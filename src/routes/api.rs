use paperclip::actix::web::{self, ServiceConfig};

use crate::{config::Configuration, handlers::users, middleware::JwtAuth};

pub fn configure_api(config: &Configuration) -> impl FnOnce(&mut ServiceConfig) {
    let jwt_pub_key = config.jwt_pub_key.clone();

    let inner = |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .wrap(JwtAuth::new(jwt_pub_key))
                .service(users::get_users)
                .service(users::create_user)
                .service(users::get_user)
                .service(users::update_user)
                .service(users::delete_user),
        );
    };

    inner
}
