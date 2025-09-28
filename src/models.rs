use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub path: String,
    pub service: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub service: String,
    pub url: String,
}
