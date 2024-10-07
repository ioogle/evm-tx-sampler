pub mod api;
pub mod model;
pub mod pkg;
pub mod sampler;
pub mod traits;

use once_cell::sync::Lazy;
use pkg::config::config::Config;
use std::sync::Arc;

pub static CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    let config_path = "config/production.toml";
    let config = Config::load(config_path).expect("Failed to load config");
    Arc::new(config)
});

#[cfg(test)]
mod tests {
    use crate::CONFIG;

    #[test]
    fn test_config() {
        assert_eq!(CONFIG.chains.len(), 1);

        let chain = CONFIG.chains.get(0).unwrap();
        assert_eq!(chain.id, 1);
        assert_eq!(chain.name, "ethereum");
        assert_eq!(chain.alias, "Ethereum Mainnet");
        assert_eq!(chain.block_explorer, "https://etherscan.io");
        println!("test_config success!");
    }
}
