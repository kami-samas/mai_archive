use actix_web::{
    middleware::Logger,
    App, HttpServer,
};use env_logger::Env;

#[macro_use]
extern crate serde_json;
extern crate dotenv;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!(target:"info", "Starting Server");
    let config = config::Config::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}