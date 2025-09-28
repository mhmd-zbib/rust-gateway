use axum::{body::Body, extract::Request, http::StatusCode, response::Response};
use reqwest::Client;

pub async fn proxy_handler(backend: String, req: Request) -> Result<Response<Body>, StatusCode> {
    let client = Client::new();
    let method = req.method().clone();
    let headers = req.headers().clone();
    let uri = req.uri().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
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
