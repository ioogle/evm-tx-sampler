use crate::sampler::sampler;
use crate::CONFIG;

#[cfg(test)]
mod tests {
    use alloy::hex::ToHexExt;

    use super::*;

    #[tokio::test]
    async fn test_transaction_samples() {
        let address_str = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
        let chain = CONFIG.chains.get(0).expect("no chain configuration found");

        let result = sampler::Sampler::transaction_samples(chain, &address_str).await;
        match result {
            Ok(transactions) => {
                assert!(!transactions.is_empty(), "transactions should not be empty");
                for (method_id, tx) in transactions {
                    if let Some(hash) = tx.hash.value() {
                        // avoid tx.hash.value().unwrap()
                        println!("{} : {}", method_id, hash.encode_hex());
                    } else {
                        println!("{} : (no hash available)", method_id);
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
