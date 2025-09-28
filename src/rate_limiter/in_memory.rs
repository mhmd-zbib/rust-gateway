use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct InMemoryRateLimiter {
    limits: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window: Duration,
}

impl InMemoryRateLimiter {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        InMemoryRateLimiter {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    pub fn check(&self, key: &str) -> bool {
        let mut limits = self.limits.lock().unwrap();
        let now = Instant::now();
        let entry = limits.entry(key.to_string()).or_insert((0, now));

        if now.duration_since(entry.1) > self.window {
            *entry = (1, now);
            true
        } else if entry.0 < self.max_requests {
            entry.0 += 1;
            true
        } else {
            false
        }
    }
}
