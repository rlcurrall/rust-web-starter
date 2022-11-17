use actix_web::{get, web::ServiceConfig, HttpRequest};

use crate::config::AppConfig;

pub fn configure_web(_config: &AppConfig) -> impl FnOnce(&mut ServiceConfig) {
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
