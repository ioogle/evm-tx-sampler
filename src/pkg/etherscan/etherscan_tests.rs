use crate::pkg::etherscan::etherscan;

#[cfg(test)]
mod tests {
    use super::*;
    use etherscan::EtherscanClient;
    use tokio;

    #[tokio::test]
    async fn test_etherscan_client() {
        let address = "0xc36442b4a4522e871399cd717abdd847ab11fe88"; // uniswap3 position nft: https://etherscan.io/address/0xc36442b4a4522e871399cd717abdd847ab11fe88
        let etherscan= etherscan::Etherscan::new().expect("Failed to create Etherscan instance");
        
        match etherscan.get_contract_info(address).await {
            Ok(contract_info) => {
                println!("{:?}", contract_info);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}
