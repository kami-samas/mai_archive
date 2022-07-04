use crate::{
    data::Data,
    db::{project::Entity as Project, user::Entity as User},
    models::Response,
};
use actix_web::{get, web, HttpResponse, Responder, Scope};
use sea_orm::EntityTrait;

pub mod user;

pub fn get_routes() -> Scope {
    web::scope("").service(info)
}

#[get("/info")]
async fn info(state: web::Data<Data>) -> Response<impl Responder> {
    let projects = Project::find().all(&state.database).await?;
    let users = User::find().all(&state.database).await?;
    Ok(HttpResponse::Ok().body(format!(
        "The projects size {} and the users size {}.",
        projects.len(),
        users.len()
    )))
}
