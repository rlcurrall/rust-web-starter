use actix_web::{get, web::ServiceConfig, HttpRequest};

use crate::config::Configuration;

pub fn configure_web(_config: &Configuration) -> impl FnOnce(&mut ServiceConfig) {
    let inner = |cfg: &mut ServiceConfig| {
        cfg.service(index);
    };

    inner
}

#[get("/")]
async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}
