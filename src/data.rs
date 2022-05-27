use sea_orm::DatabaseConnection;
use crate::config::Config;

pub struct Data {
    pub database: DatabaseConnection,
    pub config: Config,
}