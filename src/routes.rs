use crate::{
    handlers::{proxy_handler, register_handler},
    load_balancer::LoadBalancer,
    models::Config,
    registry::Registry,
};
use axum::{
    Router,
    routing::{any, post},
};
use std::sync::Arc;

pub fn create_router(registry: Arc<Registry>, config: &Config) -> Router {
    let mut router = Router::new();
    let registry_clone = Arc::clone(&registry);
    router = router.route(
        "/register",
        post(move |req| register_handler(registry_clone, req)),
    );
    for route in &config.routes {
        let load_balancer = Arc::new(LoadBalancer::new(
            route.service.clone(),
            Arc::clone(&registry),
        ));
        router = router.route(
            &route.path,
            any(move |req| proxy_handler(Arc::clone(&load_balancer), req)),
        );
    }
    router
}
