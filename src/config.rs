use dotenv::dotenv;
use std::{
    env,
    fmt::{
        Debug
    },
    str::{
        FromStr
    }
};

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub jwt_key: String,
    pub db_uri: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            port: Config::get_env_or("PORT", 3000),
            jwt_key: Config::get_env("JWT_KEY"),
            db_uri: Config::get_env("DATABASE_URI"),
        }
    }

    pub fn get_env_or<T>(var: &str, default: T) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        match env::var(var) {
            Ok(v) => v.parse::<T>().expect(&format!(
                "Unable to parse {} as {}",
                var,
                std::any::type_name::<T>()
            )),
            Err(_) => default,
        }
    }

    pub fn get_env<T>(var: &str) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        env::var(var)
            .expect(&format!("Missing environment variable {}", var))
            .parse::<T>()
            .expect(&format!(
                "Unable to parse {} as {}",
                var,
                std::any::type_name::<T>()
            ))
    }
}
