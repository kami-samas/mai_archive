use actix_web::{
    middleware::Logger,
    web::{Data},
    App, HttpServer,
};
use env_logger::Env;
use sea_orm::{ConnectOptions, Database};
use std::{
    time::Duration
};

#[macro_use]
extern crate serde_json;
extern crate dotenv;

mod config;
mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!(target:"core::server", "Starting Server");
    let config = config::Config::new();

    let mut opt = ConnectOptions::new(config.db_uri.clone());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let database = Database::connect(opt).await.unwrap();

    log::info!(target: "core::database","Connected to the database");


    let api_data = Data::new(
        data::Data {
            database,
            config: config.clone(),
        }
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(api_data.clone())
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}