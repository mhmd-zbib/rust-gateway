use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub routes: Vec<Route>,
    pub auth: Option<AuthConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub path: String,
    pub service: String,
}
