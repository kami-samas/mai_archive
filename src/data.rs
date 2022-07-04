use crate::config::Config;

pub struct Data {
    pub database: sea_orm::DatabaseConnection,
    pub config: Config,
    pub id_generator: sonyflake::Sonyflake,
}
