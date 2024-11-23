use std::sync::OnceLock;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TradingParameters {
    pub max_positions: usize,
    pub risk_tolerance: f64,
    // Add other parameters as need
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub db_connection_string: String,
    pub trading_parameters: TradingParameters,
}

pub struct ConfigManager {
    config: Config
}

impl ConfigManager {
    fn new() -> Self {
        let config_data = fs::read_to_string("config.json")
            .expect("Failed to read configuration file");
        let config: Config = serde_json::from_str(&config_data)
            .expect("Failed to parse configuration file");

        Self { config }
    }

    pub fn instance() -> &'static ConfigManager {
        static INSTANCE: OnceLock<ConfigManager> = OnceLock::new();
        INSTANCE.get_or_init(|| ConfigManager::new())
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

pub trait OrderExecution {
    fn execute(&self);
}

pub struct TWAP;

impl OrderExecution for TWAP {
    fn execute(&self) {
        let config_manager = ConfigManager::instance();
        let max_positions = config_manager.get_config().trading_parameters.max_positions;
        let risk_tolerance = config_manager.get_config().trading_parameters.risk_tolerance;
        // Implement execution logic using the configurations
        println!("max_positions={}, risk_tolerance={}", max_positions, risk_tolerance);
    }
}

fn main() {
    let strategy = TWAP;
    strategy.execute();

    // Access the ConfigManager directly elsewhere
    let api_key = ConfigManager::instance().get_config().api_key.clone();
    println!("Using API Key: {}", api_key);
}