use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Registry {
    services: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, service: String, url: String) {
        let mut services = self.services.lock().unwrap();
        services.entry(service).or_default().push(url);
    }

    pub fn get_backends(&self, service: &str) -> Vec<String> {
        let services = self.services.lock().unwrap();
        services.get(service).cloned().unwrap_or_default()
    }
}
