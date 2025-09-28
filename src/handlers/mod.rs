pub mod health;
pub mod metrics;
pub mod proxy;
pub mod register;

pub use health::health_handler;
pub use metrics::metrics_handler;
pub use proxy::proxy_handler;
pub use register::register_handler;
