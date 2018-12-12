use serde_json;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Clone, Serialize)]
pub struct Register<'a> {
    pub instance: &'a Instance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    /// This doubles as the instance ID, because why not, Eureka?
    pub host_name: String,
    pub instance_id: Option<String>,
    pub app: String,
    pub ip_addr: String,
    pub vip_address: String,
    pub secure_vip_address: String,
    pub status: StatusType,
    pub port: Option<PortData>,
    pub secure_port: PortData,
    pub home_page_url: String,
    pub status_page_url: String,
    pub health_check_url: String,
    pub data_center_info: DataCenterInfo,
    pub lease_info: Option<LeaseInfo>,
    /// optional app specific metadata
    pub metadata: Option<HashMap<String, String>>,
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            host_name: "localhost".to_string(),
            instance_id: Some(format!(
                "{}:127.0.0.1",
                env::var("CARGO_PKG_NAME").unwrap_or_default()
            )),
            app: env::var("CARGO_PKG_NAME").unwrap_or_default(),
            ip_addr: "127.0.0.1".to_string(),
            vip_address: env::var("CARGO_PKG_NAME").unwrap_or_default(),
            secure_vip_address: env::var("CARGO_PKG_NAME").unwrap_or_default(),
            status: StatusType::Starting,
            port: None,
            secure_port: PortData::new(443, false),
            home_page_url: String::new(),
            status_page_url: String::new(),
            health_check_url: String::new(),
            data_center_info: DataCenterInfo::default(),
            lease_info: None,
            metadata: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortData {
    #[serde(rename = "$")]
    value: u16,
    #[serde(rename = "@enabled")]
    enabled: String,
}

impl PortData {
    pub fn new(port: u16, enabled: bool) -> Self {
        PortData {
            value: port,
            enabled: enabled.to_string(),
        }
    }

    pub fn value(&self) -> Option<u16> {
        if self.enabled == "true" {
            Some(self.value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllApplications {
    pub applications: Applications,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Applications {
    pub application: Vec<Application>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationWrapper {
    pub application: Application,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    pub name: String,
    pub instance: Vec<Instance>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstanceWrapper {
    pub instance: Instance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCenterInfo {
    #[serde(rename = "@class")]
    class: String,
    pub name: DcNameType,
    /// metadata is only allowed if name is Amazon, and then is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AmazonMetadataType>,
}

impl Default for DataCenterInfo {
    fn default() -> Self {
        DataCenterInfo {
            class: "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo".into(),
            name: DcNameType::MyOwn,
            metadata: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaseInfo {
    /// (optional) if you want to change the length of lease - default if 90 secs
    pub eviction_duration_in_secs: Option<usize>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DcNameType {
    MyOwn,
    Amazon,
}

impl Display for DcNameType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatusType {
    Up,
    Down,
    Starting,
    OutOfService,
    Unknown,
}

impl Display for StatusType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(
            f,
            "{}",
            serde_json::to_value(self).unwrap().as_str().unwrap()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AmazonMetadataType {
    pub ami_launch_index: String,
    pub local_hostname: String,
    pub availability_zone: String,
    pub instance_id: String,
    pub public_ipv4: String,
    pub public_hostname: String,
    pub ami_manifest_path: String,
    pub local_ipv4: String,
    pub hostname: String,
    pub ami_id: String,
    pub instance_type: String,
}

#[test]
fn test_instance_json() {
    let s = "{\"applications\":{\"versions__delta\":\"1\",\"apps__hashcode\":\"UP_7_\",\"application\":[{\"name\":\"AUTH-SERVER\",\"instance\":[{\"instanceId\":\"auth-server:192.168.100.7:8000 \",\"hostName\":\"192.168.100.7\",\"app\":\"AUTH-SERVER\",\"ipAddr\":\"192.168.100.7\",\"status\":\"UP\",\"overriddenStatus\":\"UNKNOWN\",\"port\":{\"$\":8000,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":5,\"durationInSecs\":10,\"registrationTimestamp\":1544579008473,\"lastRenewalTimestamp\":1544579601047,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579008493},\"metadata\":{\"management.port\":\"8000\"},\"homePageUrl\":\"http://192.168.100.7:8000/\",\"statusPageUrl\":\"http://192.168.100.7:8000/document.html\",\"healthCheckUrl\":\"http://192.168.100.7:8000/actuator/health\",\"vipAddress\":\"auth-server\",\"secureVipAddress\":\"auth-server\",\"isCoordinatingDiscoveryServer\":\"false\",\"lastUpdatedTimestamp\":\"1544579008493\",\"lastDirtyTimestamp\":\"1544579008410\",\"actionType\":\"ADDED\"}]},{\"name\":\"AUTH-GATEWAY\",\"instance\":[{\"instanceId\":\"auth-gateway:192.168.100.6:9200\",\"hostName\":\"172.18.223.207\",\"app\":\"AUTH-GATEWAY\",\"ipAddr\":\"172.18.223.207\",\"status\":\"UP\",\"overriddenStatus\":\"UNKNOWN\",\"port\":{\"$\":9200,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":5,\"durationInSecs\":90,\"registrationTimestamp\":1544579008522,\"lastRenewalTimestamp\":1544579681050,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579008522},\"metadata\":{\"management.port\":\"9200\"},\"homePageUrl\":\"http://172.18.223.207:9200/\",\"statusPageUrl\":\"http://192.168.100.6:9200/swagger-ui.html\",\"healthCheckUrl\":\"http://172.18.223.207:9200/actuator/health\",\"vipAddress\":\"auth-gateway\",\"secureVipAddress\":\"auth-gateway\",\"isCoordinatingDiscoveryServer\":\"false\",\"lastUpdatedTimestamp\":\"1544579008522\",\"lastDirtyTimestamp\":\"1544579008479\",\"actionType\":\"ADDED\"}]},{\"name\":\"DEVICE-MANAGER\",\"instance\":[{\"instanceId\":\"device-manager:172.18.223.207:8202\",\"hostName\":\"172.18.223.207\",\"app\":\"DEVICE-MANAGER\",\"ipAddr\":\"172.18.223.207\",\"status\":\"UP\",\"overriddenStatus\":\"UNKNOWN\",\"port\":{\"$\":8202,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":5,\"durationInSecs\":10,\"registrationTimestamp\":1544579008505,\"lastRenewalTimestamp\":1544579601047,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579008505},\"metadata\":{\"management.port\":\"8202\"},\"homePageUrl\":\"http://172.18.223.207:8202/\",\"statusPageUrl\":\"http://172.18.223.207:8202/document.html\",\"healthCheckUrl\":\"http://172.18.223.207:8202/actuator/health\",\"vipAddress\":\"device-manager\",\"secureVipAddress\":\"device-manager\",\"isCoordinatingDiscoveryServer\":\"false\",\"lastUpdatedTimestamp\":\"1544579008505\",\"lastDirtyTimestamp\":\"1544579008464\",\"actionType\":\"ADDED\"}]},{\"name\":\"EUREKA-SERVER\",\"instance\":[{\"instanceId\":\"eureka-server:172.18.223.207:1111\",\"hostName\":\"172.18.223.207\",\"app\":\"EUREKA-SERVER\",\"ipAddr\":\"172.18.223.207\",\"status\":\"UP\",\"overriddenStatus\":\"UNKNOWN\",\"port\":{\"$\":1111,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":30,\"durationInSecs\":90,\"registrationTimestamp\":1544579006086,\"lastRenewalTimestamp\":1544579661800,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579006087},\"metadata\":{\"management.port\":\"1111\"},\"homePageUrl\":\"http://172.18.223.207:1111/\",\"statusPageUrl\":\"http://172.18.223.207:1111/actuator/info\",\"healthCheckUrl\":\"http://172.18.223.207:1111/actuator/health\",\"vipAddress\":\"eureka-server\",\"secureVipAddress\":\"eureka-server\",\"isCoordinatingDiscoveryServer\":\"true\",\"lastUpdatedTimestamp\":\"1544579006088\",\"lastDirtyTimestamp\":\"1544579003961\",\"actionType\":\"ADDED\"}]},{\"name\":\"USER-CENTER\",\"instance\":[{\"instanceId\":\"user-center:192.168.100.8:7002\",\"hostName\":\"192.168.100.8\",\"app\":\"USER-CENTER\",\"ipAddr\":\"192.168.100.8\",\"status\":\"UP\",\"overriddenStatus\":\"UNKNOWN\",\"port\":{\"$\":7000,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":5,\"durationInSecs\":10,\"registrationTimestamp\":1544579008533,\"lastRenewalTimestamp\":1544579601049,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579008533},\"metadata\":{\"management.port\":\"7000\"},\"homePageUrl\":\"http://192.168.100.8:7000/\",\"statusPageUrl\":\"http://192.168.100.8:7002/document.html\",\"healthCheckUrl\":\"http://192.168.100.8:7000/actuator/health\",\"vipAddress\":\"user-center\",\"secureVipAddress\":\"user-center\",\"isCoordinatingDiscoveryServer\":\"false\",\"lastUpdatedTimestamp\":\"1544579008533\",\"lastDirtyTimestamp\":\"1544579008476\",\"actionType\":\"ADDED\"}]},{\"name\":\"USER-MANAGER\",\"instance\":[{\"instanceId\":\"user-manager:172.18.223.207:8201\",\"hostName\":\"172.18.223.207\",\"app\":\"USER-MANAGER\",\"ipAddr\":\"172.18.223.207\",\"status\":\"UP\",\"overriddenStatus\":\"UNKNOWN\",\"port\":{\"$\":8201,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":5,\"durationInSecs\":10,\"registrationTimestamp\":1544579008490,\"lastRenewalTimestamp\":1544579601052,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579008490},\"metadata\":{\"management.port\":\"8201\"},\"homePageUrl\":\"http://172.18.223.207:8201/\",\"statusPageUrl\":\"http://172.18.223.207:8201/document.html\",\"healthCheckUrl\":\"http://172.18.223.207:8201/actuator/health\",\"vipAddress\":\"user-manager\",\"secureVipAddress\":\"user-manager\",\"isCoordinatingDiscoveryServer\":\"false\",\"lastUpdatedTimestamp\":\"1544579008490\",\"lastDirtyTimestamp\":\"1544579008411\",\"actionType\":\"ADDED\"}]},{\"name\":\"DEVICE-STATE\",\"instance\":[{\"instanceId\":\"device-state:172.18.220.137:8090\",\"hostName\":\"172.18.220.137\",\"app\":\"DEVICE-STATE\",\"ipAddr\":\"172.18.220.137\",\"status\":\"UP\",\"overriddenStatus\":\"UP\",\"port\":{\"$\":8090,\"@enabled\":\"true\"},\"securePort\":{\"$\":443,\"@enabled\":\"false\"},\"countryId\":1,\"dataCenterInfo\":{\"@class\":\"com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo\",\"name\":\"MyOwn\"},\"leaseInfo\":{\"renewalIntervalInSecs\":30,\"durationInSecs\":90,\"registrationTimestamp\":1544579594029,\"lastRenewalTimestamp\":1544579684041,\"evictionTimestamp\":0,\"serviceUpTimestamp\":1544579594041},\"metadata\":{\"@class\":\"java.util.Collections$EmptyMap\"},\"homePageUrl\":\"\",\"statusPageUrl\":\"\",\"healthCheckUrl\":\"\",\"vipAddress\":\"device-state\",\"secureVipAddress\":\"device-state\",\"isCoordinatingDiscoveryServer\":\"false\",\"lastUpdatedTimestamp\":\"1544579594042\",\"lastDirtyTimestamp\":\"1544579594028\",\"actionType\":\"MODIFIED\"}]}]}}";
    let all: AllApplications = serde_json::from_str(&s).unwrap();
    println!("{:?}", all);

}
