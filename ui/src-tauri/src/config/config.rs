use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigTemplate {
    pub name: String,
    pub url: String,
    pub method: String,
    pub header: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: u32,
    pub templates: Vec<ConfigTemplate>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 0,
            templates: vec![],
        }
    }
}
