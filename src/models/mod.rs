use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use regex::Regex;

pub mod user;

lazy_static! {
    pub static ref EMAIL_REGEX: regex::Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
    pub static ref USERNAME_REGEX: regex::Regex =
        Regex::new(r"^[A-Za-z][A-Za-z0-9_]{4,14}$").unwrap();
}

#[derive(Debug, Display)]
pub struct Error(anyhow::Error);
pub type Response<T> = Result<T, Error>;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log::error!("{:?}", &self.0.to_string());
        return HttpResponse::InternalServerError().body(self.0.to_string());
    }
}

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
