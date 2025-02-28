#![allow(unused, deprecated)]

extern crate itertools;
#[macro_use]
extern crate log;
extern crate percent_encoding;
#[macro_use]
extern crate quick_error;
extern crate rand;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use reqwest::{Error as ReqwestError, Method, Response, StatusCode};
use reqwest::Client as ReqwestClient;
use reqwest::header::HeaderMap;
pub use serde::de::DeserializeOwned;
pub use serde::Serialize;

pub use self::instance::{Instance, PortData, SecurePort, StatusType};
use self::instance::InstanceClient;
use self::registry::RegistryClient;

mod aws;
mod instance;
mod registry;
mod resolver;
mod rest;

/// Eureka client config
pub struct ClientConfig {
    pub eureka_connection_idle_timeout_seconds: usize,
    pub eureka_server_connect_timeout_seconds: usize,
    pub eureka_server_d_n_s_name: String,
    pub eureka_server_port: u16,
    pub eureka_server_read_timeout_seconds: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        todo!()
    }
}

pub struct EurekaInstanceConfig {}

/// Eureka configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EurekaConfig {
    /// Server host, default localhost
    pub host: String,
    /// Server port, default 8761
    pub port: u16,
    /// Heartbeat interval in milli-seconds, default 30,000
    pub heartbeat_interval: usize,
    /// Registry fetch interval in milli-seconds, default 30,000
    pub registry_fetch_interval: usize,
    /// Request max retries, default 3
    pub max_retries: usize,
    /// Eureka request retry delay in milli-seconds, default 500
    pub request_retry_delay: usize,
    /// Fetch registry or not
    pub fetch_registry: bool,
    /// Filter instance
    pub filter_up_instances: bool,
    /// Service path
    pub service_path: String,
    /// Use ssl
    pub ssl: bool,
    pub use_dns: bool,
    pub prefer_same_zone: bool,
    pub cluster_refresh_interval: usize,
    pub fetch_metadata: bool,
    pub register_with_eureka: bool,
    pub use_local_metadata: bool,
    pub prefer_ip_address: bool,
}

impl Default for EurekaConfig {
    fn default() -> Self {
        EurekaConfig {
            host: "localhost".to_string(),
            port: 8761,
            heartbeat_interval: 30_000,
            registry_fetch_interval: 30_000,
            max_retries: 3,
            request_retry_delay: 500,
            fetch_registry: true,
            filter_up_instances: true,
            service_path: "/eureka".to_string(),
            ssl: false,
            use_dns: false,
            prefer_same_zone: true,
            cluster_refresh_interval: 300_000,
            fetch_metadata: true,
            register_with_eureka: true,
            use_local_metadata: false,
            prefer_ip_address: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BaseConfig {
    pub eureka: EurekaConfig,
    pub instance: Instance,
}

quick_error! {
    #[derive(Debug)]
    pub enum EurekaError {
        Network(err: ReqwestError) {
            description(err.description())
            cause(err)
        }
        Request(status: StatusCode) {
            description(status.canonical_reason().unwrap_or("Unknown Status Code"))
        }
        UnexpectedState(description: String) {
            description(description)
        }
        ParseError(description: String) {}
    }
}

#[derive(Debug)]
pub struct EurekaClient {
    base_url: String,
    config: BaseConfig,
    client: ReqwestClient,
    registry: RegistryClient,
    instance: Option<InstanceClient>,
}

impl EurekaClient {
    pub fn new(config: BaseConfig) -> Self {
        let base_url = {
            let ssl = config.eureka.ssl;
            let protocol = if ssl { "https" } else { "http" };
            let host = &config.eureka.host;
            let port = config.eureka.port;
            let service_path = &config.eureka.service_path;
            format!("{}://{}:{}{}", protocol, host, port, service_path)
        };
        let mut instance = config.instance.clone();
        instance.vip_address = instance.app.clone();
        instance.secure_vip_address = instance.vip_address.clone();
        EurekaClient {
            base_url: base_url.clone(),
            client: ReqwestClient::new(),
            registry: RegistryClient::new(base_url.clone()),
            instance: if config.eureka.register_with_eureka {
                Some(InstanceClient::new(base_url, instance))
            } else {
                None
            },
            config,
        }
    }

    pub fn start(&self) {
        self.registry.start();
        if let Some(ref instance) = self.instance {
            instance.start();
        }
    }

    pub fn find_app_address(&self, app_id: &str) -> Option<String> {
        let instance = self.registry.get_instance_by_app_name(app_id);
        if let Some(instance) = instance {
            let ssl = self.config.eureka.ssl;
            let host = instance.ip_addr;
            let port = if ssl {
                instance.secure_port.value
            } else {
                instance.port.value
            };
            println!("app {} addr {}:{}", app_id, host, port);
            Some(format!("{}:{}", host, port))
        } else {
            None
        }
    }

    /// Sends a request to another app in this eureka cluster, and returns the response.
    ///
    /// This method assumes that your services all communicate using JSON.
    /// Future methods may be added to allow other request body types.
    ///
    /// You can add additional headers such as `Authorization` using the `headers` parameter.
    pub fn make_request<V: Serialize>(
        &self,
        app: &str,
        path: &str,
        method: Method,
        body: &V,
        mut headers: HeaderMap,
    ) -> Result<Response, EurekaError> {
        log::debug!("finding app {}", app);
        let instance = self.registry.get_instance_by_app_name(app);
        if let Some(instance) = instance {
            //println!("app {} instance {:?}", app, instance);
            let ssl = self.config.eureka.ssl;
            let protocol = if ssl { "https" } else { "http" };
            let host = instance.ip_addr;
            let port = if ssl {
                instance.secure_port.value
            } else {
                instance.port.value
            };
            log::debug!("app {} addr {}:{}", app, host, port);
            self.client
                .request(
                    method,
                    &format!(
                        "{}://{}:{}/{}",
                        protocol,
                        host,
                        port,
                        path.trim_left_matches('/')
                    ),
                )
                .headers(headers)
                .json(body)
                .send()
                .map_err(EurekaError::Network)
        } else {
            Err(EurekaError::UnexpectedState(format!(
                "Could not find app {}",
                app
            )))
        }
    }

    pub fn call<V: Serialize, R: DeserializeOwned>(
        &self,
        app: &str,
        path: &str,
        method: Method,
        body: &V,
        mut headers: HeaderMap,
    ) -> Result<R, EurekaError> {
        let mut resp = self.make_request(app, path, method, body, headers)?;
        match resp.status() {
            StatusCode::OK => Ok(resp.json().map_err(EurekaError::Network)?),
            s => Err(EurekaError::Request(resp.status())),
        }
    }
}

fn path_segment_encode(value: &str) -> String {
    percent_encoding::utf8_percent_encode(value, percent_encoding::PATH_SEGMENT_ENCODE_SET)
        .to_string()
}

fn query_encode(value: &str) -> String {
    percent_encoding::utf8_percent_encode(value, percent_encoding::QUERY_ENCODE_SET).to_string()
}
