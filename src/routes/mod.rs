use crate::data::Data;
use actix_web::{get, web, Error, HttpResponse, Scope};

pub fn get_routes() -> Scope {
    web::scope("/").service(info)
}

#[get("info")]
async fn info(state: web::Data<Data>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Info"))
}
