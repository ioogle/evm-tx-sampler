use std::str::FromStr;

use crate::pkg::config::{client::*, config::ChainConfig};
use alloy::primitives::*;
use eyre::{OptionExt, Result};
use hex::ToHexExt;

#[derive(Debug)]
pub struct ContractInfo {
    pub abi: Option<Box<String>>,
    pub address: String,
    pub chain: Box<ChainConfig>,
    pub contract_name: Option<String>,
    pub implement: Option<Box<ContractInfo>>,
    pub proxy_type: Option<String>,
    pub source_code: Option<Box<String>>,
}

#[derive(Debug)]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: u64,
    pub chain: Box<ChainConfig>,
    pub from_address: String,
    pub gas: u64,
    pub gas_price: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
    pub max_fee_per_gas: Option<u128>,
    pub hash: String,
    pub input: String,
    pub method_id: String,   // e.g. 0x88316456
    pub method_name: String, // e.g. mint(tuple params)
    pub nonce: u64,
    pub timestamp: u64,
    pub to_address: String,
    pub transaction_index: u64,
    pub transaction_type: u8,
    pub value: u128,
    pub receipt: Receipt,
}

impl Transaction {
    pub async fn new(chain_config: Box<ChainConfig>, tx_hash: &str) -> Result<Self> {
        let provider = new_rpc_client(chain_config.as_ref()).await?;

        let tx_hash_b256 = B256::from_str(tx_hash)?;
        let tx = provider.get_transaction_by_hash(tx_hash_b256).await?
        .ok_or_eyre(format!("tx hash not found: {}!", tx_hash))?;

        let block_hash = tx.block_hash.ok_or_eyre("block hash not found")?.encode_hex();
        let block_number = tx.block_number.ok_or_eyre("block number not found")?;

        let method_id = Self::method_id(&tx.input)?; 

        // hydrate: where or not including full transaction details
        let block = provider.get_block_by_number(
            alloy::eips::BlockNumberOrTag::Number(tx.block_number.ok_or_eyre("invalid block number")?), false).await?;
        let block_timestamp = block.ok_or_eyre("invalid block")?.header.timestamp;

        let mut to = String::from_str("0x")?;
        if let Some(to_raw) = tx.to {
            to = to_raw.encode_hex();
        }

        Ok(Self {
            block_hash: block_hash,
            block_number: block_number,
            chain: chain_config,
            from_address: tx.from.encode_hex(),
            gas: tx.gas,
            gas_price: tx.gas_price,
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
            max_fee_per_gas: tx.max_fee_per_gas,
            hash: String::from_str(tx_hash)?,
            input: tx.input.encode_hex(),
            method_id: method_id,
            method_name: (),
            nonce: tx.nonce,
            timestamp: block_timestamp,
            to_address: to,
            transaction_index: tx.transaction_index.ok_or_eyre("invalid tx index")?,
            transaction_type: tx.transaction_type.ok_or_eyre("invalid tx type")?,
            value: ,
            receipt: (),
        })
    }

    fn method_id(input: &Bytes) -> Result<String> {
        if input.len() < 8 {
            return Ok("0x".to_string())
        }

        let method_id_bytes = &input[..8];
        let method_id_hex = hex::encode(method_id_bytes);

        Ok(format!("0x{}", method_id_hex))
    }
}

#[derive(Debug)]
pub struct Log {
    pub address: String,
    pub chain: Box<ChainConfig>,
    pub data: String,
    pub event_id: String, // e.g. 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
    pub event_name: String, // e.g. Transfer (index_topic_1 address from, index_topic_2 address to, uint256 value)
    pub hash: String,
    pub log_index: u64,
    pub topics: Vec<Option<String>>,
}

#[derive(Debug)]
pub struct Receipt {
    pub contract_address: Option<String>, // null if contract creation
    pub cumulative_gas_used: u64,
    pub effective_gas_price: u64,
    pub gas_used: u64,
    pub hash: String,
    pub logs: Vec<Log>,
    pub status: String, // 1 - success; 0 - failed
}
