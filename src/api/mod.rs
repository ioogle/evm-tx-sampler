pub mod sample;
mod utils;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sample::sample_handler);
}