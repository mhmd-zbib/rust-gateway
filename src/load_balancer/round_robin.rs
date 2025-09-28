use crate::registry::Registry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Clone, Debug)]
struct CircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    last_failure: Option<Instant>,
    next_attempt: Option<Instant>,
}

impl CircuitBreaker {
    fn new() -> Self {
        CircuitBreaker {
            state: CircuitState::Closed,
            failure_count: 0,
            last_failure: None,
            next_attempt: None,
        }
    }
}

pub struct LoadBalancer {
    service: String,
    registry: Arc<Registry>,
    index: Arc<Mutex<usize>>,
    circuits: Arc<Mutex<HashMap<String, CircuitBreaker>>>,
    failure_threshold: u32,
    timeout: Duration,
}

impl LoadBalancer {
    pub fn new(service: String, registry: Arc<Registry>) -> Self {
        LoadBalancer {
            service,
            registry,
            index: Arc::new(Mutex::new(0)),
            circuits: Arc::new(Mutex::new(HashMap::new())),
            failure_threshold: 3, // Configurable later
            timeout: Duration::from_secs(30),
        }
    }

    pub fn next_backend(&self) -> Option<String> {
        let backends = self.registry.get_backends(&self.service);
        let mut circuits = self.circuits.lock().unwrap();
        let now = Instant::now();

        // Filter healthy backends
        let healthy_backends: Vec<String> = backends
            .into_iter()
            .filter(|backend| {
                let circuit = circuits
                    .entry(backend.clone())
                    .or_insert_with(CircuitBreaker::new);
                match circuit.state {
                    CircuitState::Closed => true,
                    CircuitState::Open => {
                        if let Some(next) = circuit.next_attempt {
                            if now >= next {
                                circuit.state = CircuitState::HalfOpen;
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    CircuitState::HalfOpen => true,
                }
            })
            .collect();

        if healthy_backends.is_empty() {
            return None;
        }

        let mut idx = self.index.lock().unwrap();
        let backend = healthy_backends[*idx % healthy_backends.len()].clone();
        *idx += 1;
        Some(backend)
    }

    pub fn report_failure(&self, backend: &str) {
        let mut circuits = self.circuits.lock().unwrap();
        let circuit = circuits
            .entry(backend.to_string())
            .or_insert_with(CircuitBreaker::new);
        let now = Instant::now();

        circuit.failure_count += 1;
        circuit.last_failure = Some(now);

        if circuit.failure_count >= self.failure_threshold {
            circuit.state = CircuitState::Open;
            circuit.next_attempt = Some(now + self.timeout);
        }
    }

    pub fn report_success(&self, backend: &str) {
        let mut circuits = self.circuits.lock().unwrap();
        if let Some(circuit) = circuits.get_mut(backend) {
            circuit.failure_count = 0;
            circuit.state = CircuitState::Closed;
            circuit.next_attempt = None;
        }
    }
}
