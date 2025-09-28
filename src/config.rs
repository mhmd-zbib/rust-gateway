use crate::models::Config;
use std::fs;

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}
