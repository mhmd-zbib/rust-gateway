use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MetricsCollector {
    metrics: Arc<Mutex<HashMap<String, u64>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        MetricsCollector {
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn increment(&self, key: &str) {
        let mut metrics = self.metrics.lock().unwrap();
        *metrics.entry(key.to_string()).or_insert(0) += 1;
    }

    pub fn all(&self) -> HashMap<String, u64> {
        let metrics = self.metrics.lock().unwrap();
        metrics.clone()
    }
}
