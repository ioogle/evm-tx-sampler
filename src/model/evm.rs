use crate::pkg::config::config::Chain;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct ContractInfo {
    pub abi: Option<Box<String>>,
    pub address: String,
    pub chain: Box<Chain>,
    pub contract_name: Option<String>,
    pub implement: Option<Box<ContractInfo>>,
    pub proxy_type: Option<String>,
    pub source_code: Option<Box<String>>,
}

#[derive(Debug)]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: u64,
    pub chain: Box<Chain>,
    pub from_address: String,
    pub gas: Decimal,
    pub gas_price: Decimal,
    pub max_priority_fee_per_gas: Decimal,
    pub max_fee_per_gas: Decimal,
    pub hash: String,
    pub input: String,
    pub method_id: String,   // e.g. 0x88316456
    pub method_name: String, // e.g. mint(tuple params)
    pub nonce: u64,
    pub timestamp: u64,
    pub to_address: String,
    pub transaction_index: u64,
    pub transaction_type: u64,
    pub value: Decimal,
    pub receipt: Receipt,
}

#[derive(Debug)]
pub struct Log {
    pub address: String,
    pub chain: Box<Chain>,
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
    pub cumulative_gas_used: Decimal,
    pub effective_gas_price: Decimal,
    pub gas_used: Decimal,
    pub hash: String,
    pub logs: Vec<Log>,
    pub status: String, // 1 - success; 0 - failed
}
