use crate::{
    handlers::{auth_middleware, health_handler, metrics_handler, proxy_handler, register_handler},
    load_balancer::LoadBalancer,
    metrics::MetricsCollector,
    models::Config,
    rate_limiter::InMemoryRateLimiter,
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
    rate_limiter: Arc<InMemoryRateLimiter>,
    config: &Config,
) -> Router {
    let mut router = Router::new();
    router = router.route("/health", get(health_handler));
    let metrics_for_handler = Arc::clone(&metrics);
    router = router.route(
        "/metrics",
        get(async move || metrics_handler(metrics_for_handler).await),
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
        let rate_limiter_clone = Arc::clone(&rate_limiter);
        let metrics_clone = Arc::clone(&metrics);
        router = router.route(
            &route.path,
            any(async move |req: Request| {
                if !rate_limiter_clone.check("global") {
                    return axum::response::Response::builder()
                        .status(axum::http::StatusCode::TOO_MANY_REQUESTS)
                        .body(axum::body::Body::from("Rate limit exceeded"))
                        .unwrap();
                }
                match proxy_handler(Arc::clone(&load_balancer), metrics_clone, req).await {
                    Ok(resp) => resp,
                    Err(status) => axum::response::Response::builder()
                        .status(status)
                        .body(axum::body::Body::empty())
                        .unwrap(),
                }
            })
            .layer(axum::middleware::from_fn(auth_middleware)),
        );
    }
    router
}
