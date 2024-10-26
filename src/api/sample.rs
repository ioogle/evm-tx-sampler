use crate::{api::utils::ResponseWrapper, pkg::config::config::ChainConfig};
use crate::sampler::sampler;
use crate::CONFIG;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SampleQuery {
    chain: String,
    address: String,
}

#[derive(Serialize)]
struct SampleItem {
    chain: String,
    tx_hash: String,
    method_id: String,
    method_signature: String,
    logs: Vec<(String, String)>, // (event id, event signature)
}

#[get("/sample")]
async fn sample_handler(query: web::Query<SampleQuery>) -> impl Responder {
    let mut response: ResponseWrapper<Vec<SampleItem>> = ResponseWrapper {
        status: 0,
        error_message: None,
        data: None,
    };

    let chain_config: ChainConfig;
    match CONFIG.chain_by_name(&query.chain) {
        Ok(cfg) => chain_config = cfg,
        Err(e) => {
            println!("{}", e);
            response.error_message = Some("error: please try it again or check the logs".to_string());
            return HttpResponse::BadRequest().json(response);
        }
    }

    let transactions = sampler::Sampler::transaction_samples(&chain_config, &query.address).await;
    match transactions {
        Ok(txs) => {
            let items: Vec<SampleItem> = txs
                .iter()
                .map(|tx| SampleItem {
                    chain: query.chain.clone(),
                    tx_hash: tx.hash.clone(),
                    method_id: tx.method_id.clone(),
                    method_signature: tx.method_signature.clone().unwrap_or("".to_string()),
                    logs: if let Some(receipt) = &tx.receipt {
                        println!("{:?}", receipt.logs);
                        receipt
                            .logs
                            .iter()
                            .filter(|log| log.address.eq_ignore_ascii_case(&query.address))
                            .map(|log| {
                                (
                                    log.event_id.to_string(),
                                    log.event_signature.clone().unwrap_or("".to_string()),
                                )
                            })
                            .collect()
                    } else {
                        Vec::new()
                    },
                })
                .collect();
            response.status = 1;
            response.data = Some(items);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            println!("{}", e);
            response.error_message = Some("error: please try it again or check the logs".to_string());
            HttpResponse::BadRequest().json(response)
        }
    }
}
