use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    server_port: u16,
    database_url: String,
}

impl AppConfig {
    fn from_env() -> anyhow::Result<Self, envy::Error> {
        envy::from_env()
    }
}
