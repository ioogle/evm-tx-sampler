use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub chains: Vec<ChainConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChainConfig {
    pub id: u64,
    pub name: String,
    pub alias: String,
    pub block_explorer: String,
    pub etherscan_api_token: String,
    pub rpc: String,
}

impl Config {
    pub fn load<P: AsRef<Path>>(default_path: P) -> Result<Self, Box<dyn Error>> {
        Self::from_env().or_else(|_| Self::from_file(default_path))
    }

    fn from_env() -> Result<Self, Box<dyn Error>> {
        let config_str = env::var("CONFIG_CONTENT").map_err(|_| "CONFIG_CONTENT not found")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let config_content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
