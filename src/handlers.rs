use axum::{Json, body::Body, extract::Request, http::StatusCode, response::Response};
use reqwest::Client;
use std::sync::Arc;

use crate::{load_balancer::LoadBalancer, models::RegisterRequest, registry::Registry};

pub async fn proxy_handler(
    load_balancer: Arc<LoadBalancer>,
    req: Request,
) -> Result<Response<Body>, StatusCode> {
    let client = Client::new();
    let method = req.method().clone();
    let headers = req.headers().clone();
    let uri = req.uri().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let backend = load_balancer
        .next_backend()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let url = format!(
        "{}{}",
        backend,
        uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("")
    );
    let response = client
        .request(method, url)
        .headers(headers)
        .body(body_bytes)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut builder = Response::builder().status(response.status());
    for (key, value) in response.headers() {
        builder = builder.header(key, value);
    }
    let body = Body::from_stream(response.bytes_stream());
    builder
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn register_handler(
    registry: Arc<Registry>,
    Json(request): Json<RegisterRequest>,
) -> Result<Response<Body>, StatusCode> {
    registry.register(request.service, request.url);
    let body = Body::from("Registered");
    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
