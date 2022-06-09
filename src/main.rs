use actix_web::{middleware::Logger, web, web::Data, App, HttpServer};
use env_logger::Env;
use figlet_rs::FIGfont;
use sea_orm::{ConnectOptions, Database};
use std::{process::Command, time::Duration};

#[macro_use]
extern crate serde_json;
extern crate dotenv;

mod config;
mod data;
mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Command::new("clear")
        .spawn()
        .expect("Couldnt execute `clear`");
    let font = FIGfont::standand().unwrap();
    println!("{}", font.convert(" Mai ").expect("FIGLET error"));
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!(target:"core::server", "Starting Server");
    let config = config::Config::new();

    let mut opt = ConnectOptions::new(config.db_uri.clone());
    opt.max_connections(50)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let database = Database::connect(opt).await.unwrap();

    log::info!(target: "core::database","Connected to the database");

    let api_data = Data::new(data::Data {
        database,
        config: config.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(api_data.clone())
            .service(
                web::scope("/")
                    .service(routes::get_routes())
                    .service(routes::user::get_routes()),
            )
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
