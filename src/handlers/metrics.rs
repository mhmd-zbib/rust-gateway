use axum::{body::Body, http::StatusCode, response::Response};
use std::sync::Arc;

use crate::metrics::MetricsCollector;

pub async fn metrics_handler(metrics: Arc<MetricsCollector>) -> Result<Response<Body>, StatusCode> {
    let data = metrics.all();
    let json = serde_json::to_string(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let body = Body::from(json);
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
