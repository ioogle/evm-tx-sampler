use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    status: String,
    message: String,
    result: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContractInfo {
    #[serde(rename = "SorceCode")]
    source_code: Option<String>,
    #[serde(rename="ABI")]
    abi: Option<String>,
    #[serde(rename = "ContractName")]
    contract_name: Option<String>,
}