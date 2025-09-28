use axum::{body::Body, extract::Request, http::StatusCode, response::Response};
use reqwest::Client;
use std::sync::Arc;

use crate::load_balancer::LoadBalancer;

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

    for attempt in 0..3 {
        let backend = match load_balancer.next_backend() {
            Some(b) => b,
            None => {
                return Err(StatusCode::SERVICE_UNAVAILABLE);
            }
        };
        let url = format!(
            "{}{}",
            backend,
            uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("")
        );
        match client
            .request(method.clone(), url)
            .headers(headers.clone())
            .body(body_bytes.clone())
            .send()
            .await
        {
            Ok(response) => {
                let mut builder = Response::builder().status(response.status());
                for (key, value) in response.headers() {
                    builder = builder.header(key, value);
                }
                let body = Body::from_stream(response.bytes_stream());
                return builder
                    .body(body)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
            }
            Err(_) => {
                if attempt == 2 {
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                continue;
            }
        }
    }
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
