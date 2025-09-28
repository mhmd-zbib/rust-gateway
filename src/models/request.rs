use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub service: String,
    pub url: String,
}
