use std::str::FromStr;

use crate::pkg::config::{client::*, config::ChainConfig};
use crate::pkg::proxy::proxy::ProxyDetector;
use alloy::{primitives::*, providers::Provider};
use eyre::{OptionExt, Result};
use hex::ToHexExt;
use map::hash_map::HashMap;

use cached::proc_macro::cached;
use cached::SizedCache;

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
    pub chain: ChainConfig,
    pub from_address: String,
    pub gas: u64,
    pub gas_price: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
    pub max_fee_per_gas: Option<u128>,
    pub hash: String,
    pub input: String,
    pub method_id: String,                // e.g. 0x88316456
    pub method_signature: Option<String>, // e.g. mint(tuple params)
    pub nonce: u64,
    pub timestamp: u64,
    pub to_address: String,
    pub transaction_index: u64,
    pub transaction_type: u8,
    pub value: String,
    pub receipt: Option<Receipt>,
}

impl Transaction {
    pub async fn new(chain_config: &ChainConfig, tx_hash: &str) -> Result<Self> {
        let provider = new_rpc_client(chain_config).await?;

        let tx_hash_b256 = B256::from_str(tx_hash)?;
        let tx = provider
            .get_transaction_by_hash(tx_hash_b256)
            .await?
            .ok_or_eyre(format!("tx hash not found: {}!", tx_hash))?;

        let block_hash = format!(
            "0x{}",
            tx.block_hash
                .ok_or_eyre("block hash not found")?
                .encode_hex()
        );
        let block_number = tx.block_number.ok_or_eyre("block number not found")?;

        let method_id = Self::method_id(&tx.input)?;

        // hydrate: where or not including full transaction details
        let block = provider
            .get_block_by_number(
                alloy::eips::BlockNumberOrTag::Number(
                    tx.block_number.ok_or_eyre("invalid block number")?,
                ),
                false,
            )
            .await?;
        let block_timestamp = block.ok_or_eyre("invalid block")?.header.timestamp;

        let mut to = String::from_str("0x")?;
        if let Some(to_raw) = tx.to {
            to = format!("0x{}", to_raw.encode_hex());
        }

        let proxy_detector = ProxyDetector::new(chain_config).await?;
        let proxy = proxy_detector.detect_proxy_target(to.as_ref()).await?;

        let mut impl_address = tx.to.ok_or_eyre("empty to")?;
        if let Some(addr) = proxy.target {
            impl_address = addr;
        }

        let (function_map, event_map) =
            function_event_map(chain_config, &impl_address).await?;
        let method_signature = function_map.get(&method_id).cloned();

        let receipt = Receipt::new(provider, tx_hash, &event_map).await?;

        Ok(Self {
            block_hash: block_hash,
            block_number: block_number,
            chain: chain_config.clone(),
            from_address: format!("0x{}", tx.from.encode_hex()),
            gas: tx.gas,
            gas_price: tx.gas_price,
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
            max_fee_per_gas: tx.max_fee_per_gas,
            // hash: format!("0x{}", String::from_str(tx_hash)?),
            hash: String::from_str(tx_hash)?,
            input: tx.input.encode_hex(),
            method_id: method_id,
            method_signature: method_signature,
            nonce: tx.nonce,
            timestamp: block_timestamp,
            to_address: to,
            transaction_index: tx.transaction_index.ok_or_eyre("invalid tx index")?,
            transaction_type: tx.transaction_type.ok_or_eyre("invalid tx type")?,
            value: tx.value.to_string(),
            receipt: receipt,
        })
    }

    fn method_id(input: &Bytes) -> Result<String> {
        if input.len() < 4 {
            return Ok("0x".to_string());
        }
        let method_id_bytes = &input[..4];

        Ok(format!("0x{}", hex::encode(method_id_bytes)))
    }
}

#[derive(Debug)]
pub struct Log {
    pub address: String,
    pub data: String,
    pub event_id: String, // e.g. 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
    pub event_signature: Option<String>, // e.g. Transfer (index_topic_1 address from, index_topic_2 address to, uint256 value)
    pub log_index: Option<u64>,
    pub topics: Vec<String>,
}

#[derive(Debug)]
pub struct Receipt {
    pub contract_address: Option<String>, // null if contract creation
    pub effective_gas_price: u128,
    pub gas_used: u128,
    pub hash: String,
    pub logs: Vec<Log>,
    pub status: bool,
}

impl Receipt {
    pub async fn new(
        provider: Box<dyn Provider>,
        tx_hash: &str,
        event_map: &HashMap<String, String>,
    ) -> Result<Option<Receipt>> {
        let tx_hash_b256 = B256::from_str(tx_hash)?;
        let receipt_option = provider.get_transaction_receipt(tx_hash_b256).await?;

        let receipt = match receipt_option {
            Some(r) => r,
            None => return Ok(None),
        };

        let contract_address = receipt.contract_address.map(|addr| format!("{:#x}", addr));

        let logs = receipt
            .inner
            .logs()
            .into_iter()
            .map(|log| {
                let event_id = log
                    .topic0()
                    .map(|topic| format!("{:#x}", topic))
                    .unwrap_or_else(|| "0x0".to_string());

                let event_signature = event_map.get(&event_id).cloned();
                println!("{} {:?} {:?}", event_id, event_signature, event_map);

                Log {
                    address: format!("{:#x}", log.address()),
                    data: hex::encode(log.data().data.clone()),
                    event_id: event_id,
                    event_signature: event_signature,
                    log_index: log.log_index,
                    topics: log
                        .topics()
                        .into_iter()
                        .map(|topic| format!("{:#x}", topic))
                        .collect(),
                }
            })
            .collect();

        Ok(Some(Self {
            contract_address: contract_address,
            effective_gas_price: receipt.effective_gas_price,
            gas_used: receipt.gas_used,
            hash: format!("{:#x}", receipt.transaction_hash),
            logs: logs,
            status: receipt.status(),
        }))
    }
}

#[cached(
    ty = "SizedCache<String, (HashMap<String, String>, HashMap<String, String>)>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}{}", chain_config.name, address.encode_hex()) }"#,
    result = true,
)]
pub async fn function_event_map(
    chain_config: &ChainConfig,
    address: &Address,
) -> Result<(HashMap<String, String>, HashMap<String, String>)> {
    let scan = new_scan_client(chain_config)?;
    let abi = scan.contract_abi(address.clone()).await?;

    let mut function_map = HashMap::new();
    let mut event_map = HashMap::new();

    for item in abi.functions() {
        let signature = item.full_signature();
        let selector = item.selector();
        let selector_hex = format!("0x{}", hex::encode(&selector));
        function_map.insert(selector_hex, signature);
    }

    for item in abi.events() {
        let signature = item.full_signature();
        let selector = item.selector();
        let selector_hex = format!("0x{}", hex::encode(&selector));
        event_map.insert(selector_hex, signature);
    }

    Ok((function_map, event_map))
}
