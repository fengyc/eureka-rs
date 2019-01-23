pub use rest::structures::{Instance, PortData, StatusType};
use rest::EurekaRestClient;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use EurekaError;

#[derive(Debug)]
pub struct InstanceClient {
    client: Arc<EurekaRestClient>,
    config: Arc<Instance>,
    is_running: Arc<AtomicBool>,
}

impl InstanceClient {
    pub fn new(base_url: String, config: Instance) -> Self {
        InstanceClient {
            client: Arc::new(EurekaRestClient::new(base_url)),
            config: Arc::new(config),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn get_instance_id(&self) -> String {
        let mut instance_id = self.config.host_name.clone();
        if let Some(ref inst_id) = self.config.instance_id {
            instance_id = inst_id.clone();
        }
        instance_id
    }

    pub fn start(&self) {
        while let Err(e) = self.client.register(&self.config.app, &*self.config) {
            error!("Failed to register app: {}", e);
            thread::sleep(Duration::from_secs(15));
        }
        debug!("Registered app with eureka");

        self.is_running.store(true, Ordering::Relaxed);

        let is_running = Arc::clone(&self.is_running);
        let client = Arc::clone(&self.client);
        let config = Arc::clone(&self.config);
        let instance_id = self.get_instance_id();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(30));
            while is_running.load(Ordering::Relaxed) {
                let resp = client.send_heartbeat(&config.app, &instance_id);
                match resp {
                    Err(EurekaError::UnexpectedState(_)) => {
                        warn!("App not registered with eureka, reregistering");
                        let _ = client.register(&config.app, &*config);
                    }
                    Err(e) => {
                        error!("Failed to send heartbeat: {}, reregistering", e);
                        let _ = client.register(&config.app, &*config);
                    }
                    Ok(_) => {
                        debug!("Sent heartbeat successfully");
                    }
                }
                thread::sleep(Duration::from_secs(30));
            }
        });

        while let Err(e) = self.client.update_status(
            &self.config.app,
            &self.get_instance_id(),
            StatusType::Up,
        )
        {
            error!("Failed to set app to UP: {}", e);
            thread::sleep(Duration::from_secs(15));
        }
    }
}

impl Drop for InstanceClient {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        let _ = self.client.deregister(
            &self.config.app,
            &self.get_instance_id(),
        );
    }
}
