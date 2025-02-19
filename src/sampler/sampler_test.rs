use crate::sampler::sampler;
use crate::CONFIG;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transaction_samples() {
        let address_str = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88";
        let chain = CONFIG.chains.get(0).expect("no chain configuration found");

        let result = sampler::Sampler::transaction_samples(chain, &address_str).await;
        match result {
            Ok(transactions) => {
                assert!(!transactions.is_empty(), "transactions should not be empty");
                for tx in transactions {
                    println!("{} {} {:?}", tx.hash, tx.method_id, tx.method_signature,);
                    if let Some(receipt) = tx.receipt {
                        for log in receipt.logs {
                            println!("{} {} {:?}", log.address, log.event_id, log.event_signature);
                        }
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
