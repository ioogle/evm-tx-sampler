use crate::model::evm;
use crate::pkg::config::config::Chain;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::signers::k256::elliptic_curve::rand_core::block;
use eyre::Result;
use foundry_block_explorers::account::{NormalTransaction, Sort, TxListParams};
use foundry_block_explorers::Client;
use std::collections::HashMap;

pub struct Sampler {}

impl Sampler {
    pub async fn transaction_samples(
        chain: &Chain,
        address: &str,
    ) -> Result<HashMap<String, NormalTransaction>> {
        let provider = Self::new_rpc_client(chain).await?;
        let block_number = provider.get_block_number().await?;

        let scan_client  = Self::new_scan_client(chain)?;
        let transactions = scan_client
            .get_transactions(
                &address.parse()?,
                Some(TxListParams::new(0, block_number, 1, 1000, Sort::Desc)),
            )
            .await?;

        let mut result = HashMap::<String, NormalTransaction>::new();
        for tx in transactions {
            if tx.input.is_empty() {
                // skip native transfer
                continue;
            }

            if let Some(method_id) = &tx.method_id {
                // important!!! iterate the reference of tx.method_id to avoid ownership transfer
                let method_id_str = method_id.to_string();
                if !result.contains_key(&method_id_str) {
                    // if not exist, insert
                    result
                        .entry(method_id_str.clone())
                        .or_insert_with(|| tx.clone());
                }
            }
        }

        Ok(result)
    }

    pub fn new_scan_client(chain: &Chain) -> Result<Box<Client>> {
        let client = Client::new(
            alloy_chains::Chain::from_id(chain.id),
            chain.etherscan_api_token.clone(),
        )?;

        Ok(Box::new(client))
    }

    pub async fn new_rpc_client(chain: &Chain) -> Result<Box<dyn Provider>> {
        let provider = ProviderBuilder::new().on_builtin(&chain.rpc).await?;

        Ok(Box::new(provider))
    }
}
