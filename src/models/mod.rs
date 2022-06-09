use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

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
