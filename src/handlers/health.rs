use axum::{body::Body, http::StatusCode, response::Response};

pub async fn health_handler() -> Result<Response<Body>, StatusCode> {
    let body = Body::from("OK");
    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
