use itertools::Itertools;
use rand::random;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use rest::structures::{Instance, StatusType};
use rest::EurekaRestClient;

#[derive(Debug)]
pub struct RegistryClient {
    client: Arc<EurekaRestClient>,
    app_cache: Arc<RwLock<HashMap<String, Vec<Instance>>>>,
    is_running: Arc<AtomicBool>,
}

impl RegistryClient {
    pub fn new(base_url: String) -> Self {
        RegistryClient {
            client: Arc::new(EurekaRestClient::new(base_url)),
            app_cache: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn update_app_cache(&self) -> Result<(), String> {
        RegistryClient::update_app_cache_internal(&self.client, &self.app_cache)
    }

    fn update_app_cache_internal(
        client: &Arc<EurekaRestClient>,
        app_cache: &Arc<RwLock<HashMap<String, Vec<Instance>>>>,
    ) -> Result<(), String> {
        let resp = client.get_all_instances();
        match resp {
            Ok(instances) => {
                // println!("got instances {:?}", instances);
                *app_cache.write().unwrap() = group_instances_by_app(instances);
                return Ok(());
            }
            Err(e) => {
                return Err(format!("Failed to fetch registry: {:?}", e));
            }
        };
    }
    pub fn start(&self) {
        self.is_running.store(true, Ordering::Relaxed);

        let is_running = Arc::clone(&self.is_running);
        let client = Arc::clone(&self.client);
        let app_cache = Arc::clone(&self.app_cache);
        self.update_app_cache();
        thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {
                RegistryClient::update_app_cache_internal(&client, &app_cache)
                    .map_err(|e| println!("{}", e));
                thread::sleep(Duration::from_secs(30));
            }
        });
    }

    pub fn get_instance_by_app_name(&self, app: &str) -> Option<Instance> {
        // Clone the result to avoid holding onto a lock on the app cache indefinitely
        self.app_cache
            .read()
            .unwrap()
            .get(app)
            .and_then(|instances| {
                //random select one UP node
                let mut valid_ids: Vec<usize> = Vec::new();
                for i in 0..instances.len() {
                    if instances[i].status == StatusType::Up {
                        valid_ids.push(i);
                    }
                }
                if valid_ids.len() > 0 {
                    let index = valid_ids[random::<usize>() % valid_ids.len()];
                    instances.get(index)
                } else {
                    None
                }
            })
            .cloned()
    }
}

impl Drop for RegistryClient {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}

fn group_instances_by_app(instances: Vec<Instance>) -> HashMap<String, Vec<Instance>> {
    instances
        .into_iter()
        .group_by(|i| i.app.clone())
        .into_iter()
        .map(|(k, g)| (k, g.collect()))
        .collect()
}
