use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct ContractInfo {
    pub abi: Option<Box<String>>,
    pub address: String,
    pub chain: String,
    pub contract_name: Option<String>,
    pub implement: Option<Box<ContractInfo>>,
    pub proxy_type: Option<String>,
    pub source_code: Option<Box<String>>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: u64,
    pub chain: String,
    pub from_address: String,
    pub gas: Decimal,
    pub gas_price: Decimal,
    pub max_priority_fee_per_gas: Decimal,
    pub max_fee_per_gas: Decimal,
    pub hash: String,
    pub input: String,
    pub method: String,
    pub nonce: u64,
    pub timestamp: u64,
    pub to_address: String,
    pub transaction_index: u64,
    pub transaction_type: u64,
    pub value: Decimal,
}

#[derive(Debug, Clone)]
pub struct Log {
    pub address: String,
    pub chain: String,
    pub data: String,
    pub hash: String,
    pub log_index: u64,
    pub topics: Vec<Option<String>>,
}

#[derive(Debug, Clone)]
pub struct Receipt {
    pub contract_address: Option<String>, // null if contract creation
    pub cumulative_gas_used: Decimal,
    pub effective_gas_price: Decimal,
    pub gas_used: Decimal,
    pub hash: String,
    pub logs: Vec<Log>,
    pub status: String, // 1 - success; 0 - failed
}