use actix_web::{App, HttpServer};
use evm_tx_sampler::api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::init_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
