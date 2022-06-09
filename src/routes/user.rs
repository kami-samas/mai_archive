use crate::models::Response;
use actix_web::{get, web, HttpResponse, Responder, Scope};

pub fn get_routes() -> Scope {
    web::scope("/user").service(routes).service(info)
}

#[get("")]
async fn routes() -> Response<impl Responder> {
    Ok(HttpResponse::Ok().body("Routes"))
}

#[get("/{application_id}")]
async fn info(path: web::Path<(String,)>) -> Response<impl Responder> {
    let user_id = path.into_inner().0;
    Ok(HttpResponse::Ok().body(format!("{}", user_id)))
}
