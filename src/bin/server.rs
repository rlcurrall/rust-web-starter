use actix_web::{middleware, web::Data, App, HttpServer};
use app::{
    config::get_config,
    db::{establish_connection, run_migrations},
    logging::init_tracing,
    routes::{api::configure_api, web::configure_web},
};
use dotenvy::dotenv;
use paperclip::actix::OpenApiExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_config = get_config();

    init_tracing(&app_config.log.level, &app_config.log.format);

    let db_pool = establish_connection(&app_config.database_url)
        .await
        .expect("Failed to create database connection pool.");

    let workers = app_config.server.workers.clone();
    let addr = (
        app_config.server.addr.clone(),
        app_config.server.port.clone(),
    );

    run_migrations(&db_pool)
        .await
        .expect("Could not run migrations");

    HttpServer::new(move || {
        let api_conf = configure_api(&app_config);
        let web_conf = configure_web(&app_config);

        App::new()
            // Register app data
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(app_config.clone()))
            // Register global middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(middleware::Compress::default())
            // Register API routes and build documentation
            .wrap_api()
            .configure(api_conf)
            .with_json_spec_at("/openapi/v2.json")
            .with_json_spec_v3_at("/openapi/v3.json")
            .with_swagger_ui_at("/openapi")
            .build()
            // Register web routes
            .configure(web_conf)
    })
    .workers(workers)
    .bind(addr)?
    .run()
    .await
}
