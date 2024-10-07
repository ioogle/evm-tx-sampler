use crate::pkg::config::{client::*, config::ChainConfig};
use eyre::Result;
use foundry_block_explorers::account::{NormalTransaction, Sort, TxListParams};
use std::collections::HashMap;

pub struct Sampler {}

impl Sampler {
    pub async fn transaction_samples(
        chain: &ChainConfig,
        address: &str,
    ) -> Result<HashMap<String, NormalTransaction>> {
        let provider = new_rpc_client(chain).await?;
        let block_number = provider.get_block_number().await?;

        let scan_client = new_scan_client(chain)?;
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
}
