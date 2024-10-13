use std::iter::Once;

use crate::api::utils::ResponseWrapper;
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
}

#[get("/sample")]
async fn sample_handler(query: web::Query<SampleQuery>) -> impl Responder {
    let mut response: ResponseWrapper<Vec<SampleItem>> = ResponseWrapper {
        status: 0,
        error_message: None,
        data: None,
    };

    let chain_config = CONFIG.chains.get(0).expect("chain config not found");
    let transactions = sampler::Sampler::transaction_samples(&chain_config, &query.address).await;
    match transactions {
        Ok(txs) => {
            let items: Vec<SampleItem> = txs.iter().map(|tx| SampleItem{
                chain: query.chain.clone(),
                tx_hash: tx.hash.clone(),
                method_id: tx.method_id.clone(),
                method_signature: tx.method_signature.clone().unwrap_or_default(),
            }).collect();
            response.status = 1;
            response.data = Some(items);
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            response.error_message = Some(format!("{}", e));
            HttpResponse::BadRequest().json(response)
        }
    }
}
