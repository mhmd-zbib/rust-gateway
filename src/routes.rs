use crate::{handlers::proxy_handler, models::Config};
use axum::{Router, routing::any};

pub fn create_router(config: &Config) -> Router {
    let mut router = Router::new();
    for route in &config.routes {
        let backend = route.backend.clone();
        router = router.route(
            &route.path,
            any(move |req| proxy_handler(backend.clone(), req)),
        );
    }
    router
}
