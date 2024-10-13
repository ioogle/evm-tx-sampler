use crate::pkg::config::{client::*, config::ChainConfig};
use crate::model::evm::Transaction;
use alloy::hex::ToHexExt;
use eyre::Result;
use foundry_block_explorers::account::{NormalTransaction, Sort, TxListParams};
use std::collections::HashMap;

pub struct Sampler {}

impl Sampler {
    pub async fn transaction_samples(
        chain: &ChainConfig,
        address: &str,
    ) -> Result<Vec<Transaction>> {
        let provider = new_rpc_client(chain).await?;
        let block_number = provider.get_block_number().await?;

        let scan_client = new_scan_client(chain)?;
        let transactions = scan_client
            .get_transactions(
                &address.parse()?,
                Some(TxListParams::new(0, block_number, 1, 1000, Sort::Desc)),
            )
            .await?;

        let mut filtered_transactions= HashMap::<String, NormalTransaction>::new();
        for tx in transactions {
            if tx.input.is_empty() {
                // skip native transfer
                continue;
            }

            if let Some(method_id) = &tx.method_id {
                // important!!! iterate the reference of tx.method_id to avoid ownership transfer
                let method_id_str = method_id.to_string();
                if !filtered_transactions.contains_key(&method_id_str) {
                    // if not exist, insert
                   filtered_transactions 
                        .entry(method_id_str.clone())
                        .or_insert_with(|| tx.clone());
                }
            }
        }

        let mut sorted_transactions: Vec<&NormalTransaction> = filtered_transactions.values().collect();
        sorted_transactions.sort_by_key(|obj| obj.block_number.as_number());

        let mut result: Vec<Transaction> = vec![];
        for t in sorted_transactions {
            let converted = Transaction::new(chain, t.hash.value().expect("tx hash not found").encode_hex().as_str()).await?;
            result.push(converted);
        }

        Ok(result)
    }
}
