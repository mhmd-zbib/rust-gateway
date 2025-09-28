mod config;
mod handlers;
mod models;
mod routes;

use crate::{config::load_config, routes::create_router};

#[tokio::main]
async fn main() {
    let config = load_config().expect("Failed to load config");
    let app = create_router(&config);
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}