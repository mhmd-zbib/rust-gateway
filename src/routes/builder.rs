use crate::{
    handlers::{health_handler, metrics_handler, proxy_handler, register_handler},
    load_balancer::LoadBalancer,
    metrics::MetricsCollector,
    models::Config,
    registry::Registry,
};
use axum::{
    Json, Router,
    extract::Request,
    routing::{any, get, post},
};
use std::sync::Arc;

pub fn create_router(
    registry: Arc<Registry>,
    metrics: Arc<MetricsCollector>,
    config: &Config,
) -> Router {
    let mut router = Router::new();
    router = router.route("/health", get(health_handler));
    let metrics_clone = Arc::clone(&metrics);
    router = router.route(
        "/metrics",
        get(async move || metrics_handler(metrics_clone).await),
    );
    let registry_clone = Arc::clone(&registry);
    router = router.route(
        "/register",
        post(async move |Json(request)| register_handler(registry_clone, Json(request)).await),
    );
    for route in &config.routes {
        let load_balancer = Arc::new(LoadBalancer::new(
            route.service.clone(),
            Arc::clone(&registry),
        ));
        router = router.route(
            &route.path,
            any(async move |req: Request| proxy_handler(Arc::clone(&load_balancer), req).await),
        );
    }
    router
}
