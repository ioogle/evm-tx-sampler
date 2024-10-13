use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::api::utils::ResponseWrapper;

#[derive(Deserialize)]
pub struct SampleQuery {
    chain: String,
    address: String,
}

#[derive(Serialize)]
struct SampleItem {
    chain: String,
    tx_hash: String,
}

#[get("/sample")]
async fn sample_handler(query: web::Query<SampleQuery>) -> impl Responder {
    let sample_list = vec![
        SampleItem {chain: query.chain.to_string(),tx_hash: "xxx".to_string()},
    ];

    let response = ResponseWrapper{
        status: 1,
        data: Some(sample_list),
    };

    HttpResponse::Ok().json(response)
}