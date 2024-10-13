use crate::model::evm;

#[cfg(test)]
mod tests {
    use core::panic;
    use std::str::FromStr;

    use alloy::primitives::Address;

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

    #[tokio::test]
    async fn test_function_event_map() {
        let chain_config = CONFIG.chains.get(0).expect("chain config not found");
        let address = match Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48") {
            Ok(addr) => addr,
            Err(e) => panic!("{}", e)
        };
        let (functions, events) =match evm::function_event_map(chain_config, &address).await {
            Ok((functions, events)) => (functions, events),
            Err(e) => panic!("{}", e)
        };
        assert!(!functions.is_empty());
        assert!(!events.is_empty());
        println!("{:?} {:?}", functions, events);
    }
}
