use crate::registry::Registry;
use std::sync::{Arc, Mutex};

pub struct LoadBalancer {
    service: String,
    registry: Arc<Registry>,
    index: Arc<Mutex<usize>>,
}

impl LoadBalancer {
    pub fn new(service: String, registry: Arc<Registry>) -> Self {
        LoadBalancer {
            service,
            registry,
            index: Arc::new(Mutex::new(0)),
        }
    }

    pub fn next_backend(&self) -> Option<String> {
        let backends = self.registry.get_backends(&self.service);
        if backends.is_empty() {
            return None;
        }
        let mut idx = self.index.lock().unwrap();
        let backend = backends[*idx % backends.len()].clone();
        *idx += 1;
        Some(backend)
    }
}
