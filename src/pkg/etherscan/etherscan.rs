use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    status: String,
    message: String,
    result: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContractInfo {
    #[serde(rename = "SourceCode")]
    source_code: Option<String>,
    #[serde(rename = "ABI")]
    abi: Option<String>,
    #[serde(rename = "ContractName")]
    contract_name: Option<String>,
}

#[async_trait]
pub trait EtherscanClient {
    async fn get_contract_info(&self, address: &str) -> Result<ContractInfo, Box<dyn std::error::Error>>;
}

pub struct Etherscan {
    api_key: String,
    base_url: String,
}

#[derive(Deserialize)]
struct Env {
    etherscan_api_token: String,
}

impl Etherscan {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let env = envy::from_env::<Env>()?;
        Ok(Etherscan {
            api_key: env.etherscan_api_token,
            base_url: "https://api.etherscan.io/api".to_string(),
        })
    }
}

#[async_trait]
impl EtherscanClient for Etherscan {
    async fn get_contract_info(&self, address: &str) -> Result<ContractInfo, Box<dyn std::error::Error>> {
        let url = format!(
            "{}?module=contract&action=getsourcecode&address={}&apikey={}",
            self.base_url, address, self.api_key,
        );

        let response: ApiResponse<Vec<ContractInfo>> = reqwest::get(&url).await?.json().await?;

        if response.status != "1" {
            return Err(format!("response status is not 1: {:?}", response).into());
        }

        Ok(response.result[0].clone())
    }
}
