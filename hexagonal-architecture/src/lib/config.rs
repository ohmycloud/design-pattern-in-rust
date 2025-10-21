use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self, envy::Error> {
        // Load environment variables from .env file
        dotenvy::dotenv().ok();
        envy::from_env()
    }
}
