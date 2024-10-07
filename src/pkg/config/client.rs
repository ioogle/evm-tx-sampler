use crate::pkg::config::config::ChainConfig;
use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;
use foundry_block_explorers::Client;

pub fn new_scan_client(chain: &ChainConfig) -> Result<Box<Client>> {
    let client = Client::new(
        alloy_chains::Chain::from_id(chain.id),
        chain.etherscan_api_token.clone(),
    )?;

    Ok(Box::new(client))
}

pub async fn new_rpc_client(chain: &ChainConfig) -> Result<Box<dyn Provider>> {
    let provider = ProviderBuilder::new().on_builtin(&chain.rpc).await?;

    Ok(Box::new(provider))
}
