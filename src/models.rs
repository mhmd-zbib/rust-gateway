use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub port: u16,
    pub routes: Vec<Route>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    pub path: String,
    pub backend: String,
}
