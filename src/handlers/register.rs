use axum::{Json, body::Body, http::StatusCode, response::Response};
use std::sync::Arc;

use crate::{models::RegisterRequest, registry::Registry};

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
