use crate::{
    data::Data,
    db::user as User,
    util::user::gen_token,
    models::{user::UserCreateForm, Response, EMAIL_REGEX, USERNAME_REGEX},
};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub fn get_routes() -> Scope {
    web::scope("/user")
        .service(routes)
        .service(info)
        .service(create)
}

#[get("")]
async fn routes() -> Response<impl Responder> {
    Ok(HttpResponse::Ok().body("Routes"))
}

#[get("/{user_id}")]
async fn info(path: web::Path<(String,)>, state: web::Data<Data>) -> Response<impl Responder> {
    let user_id = path.into_inner().0;
    println!("{}", &state.id_generator.clone().next_id().unwrap());
    match User::Entity::find_by_id(user_id)
        .one(&state.database)
        .await?
    {
        Some(val) => {
            Ok(HttpResponse::Ok().json(json!({
                "id": val.id,
                "email": val.email
            })))
        }
        None => Ok(HttpResponse::NotFound().body("Couldnt find user")),
    }
}

#[post("")]
async fn create(
    state: web::Data<Data>,
    form: web::Json<UserCreateForm>,
) -> Response<impl Responder> {
    let mut id_gen = state.id_generator.clone();
    if !EMAIL_REGEX.is_match(&form.email) {
        return Ok(
            HttpResponse::BadRequest().body("Invalid Email. Please use a valid email address.")
        );
    }
    if !USERNAME_REGEX.is_match(&form.username) {
        return Ok(HttpResponse::BadRequest()
            .body("Invalid Username. Remove special characters and try again"));
    }

    if User::Entity::find()
        .filter(User::Column::Email.eq(form.email.to_owned()))
        .one(&state.database)
        .await?
        .is_some()
    {
        return Ok(HttpResponse::BadRequest().body("Email already exists"));
    }

    if User::Entity::find()
        .filter(User::Column::Username.eq(form.username.to_owned()))
        .one(&state.database)
        .await?
        .is_some()
    {
        return Ok(HttpResponse::BadRequest().body("Username already exists"));
    }
    let pass = match gen_token(&form.password) {
        Ok(val) => val,
        Err(err) => {
            return Ok(HttpResponse::InternalServerError().body(err.to_string()));
        }
    };
    let user_data = User::ActiveModel {
        id: Set(id_gen.next_id()?.to_string()),
        username: Set(form.username.to_owned()),
        email: Set(form.email.to_owned()),
        token: Set(pass),
        ..Default::default()
    }
    .insert(&state.database)
    .await?;

    Ok(HttpResponse::Ok().json(json!({
            "id": user_data.id,
            "email": user_data.email,
            "username": user_data.username
    })))

}
