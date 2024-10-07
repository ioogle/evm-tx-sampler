use crate::model::evm;
use async_trait::async_trait;

#[async_trait]
pub trait Client {
    async fn get_contract_info(
        &self, 
        address: &str
    ) -> Result<evm::ContractInfo, Box<dyn std::error::Error>>;

    async fn get_transaction_list(
        &self, 
        address: &str, 
        page: &u32, 
        limit: &u32
    ) -> Result<Vec<evm::Transaction>, Box<dyn std::error::Error>>;
}
