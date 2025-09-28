mod config;
mod handlers;
mod load_balancer;
mod metrics;
mod models;
mod rate_limiter;
mod registry;
mod routes;

use crate::{
    config::load_config, metrics::MetricsCollector, rate_limiter::InMemoryRateLimiter,
    registry::Registry, routes::create_router,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = load_config().expect("Failed to load config");
    let registry = Arc::new(Registry::new());
    let metrics = Arc::new(MetricsCollector::new());
    let rate_limiter = Arc::new(InMemoryRateLimiter::new(100, 60)); // 100 requests per 60 seconds
    let app = create_router(registry, metrics, rate_limiter, &config);
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
