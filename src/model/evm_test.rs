use crate::model::evm;

#[cfg(test)]
mod tests {
    use crate::CONFIG;

    use super::*;

    #[tokio::test]
    async fn test_transaction_new() {
        let chain_config = CONFIG.chains.get(0).expect("chain config not found");
        let tx_hash = "0xfaefcf34bca70cbdbe605864c6651974906f8e8b117c8b7b8c79cdaded52f56b";
        let result = evm::Transaction::new(chain_config, &tx_hash).await;
        match result {
            Ok(tx) => {
                assert_eq!(tx.hash, tx_hash);
                println!("{:?}", tx);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
