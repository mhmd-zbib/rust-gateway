mod config;
mod handlers;
mod load_balancer;
mod metrics;
mod models;
mod registry;
mod routes;

use crate::{
    config::load_config, metrics::MetricsCollector, registry::Registry, routes::create_router,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = load_config().expect("Failed to load config");
    let registry = Arc::new(Registry::new());
    let metrics = Arc::new(MetricsCollector::new());
    let app = create_router(registry, metrics, &config);
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
