use crate::{
    data::Data,
    db::user as User,
    util::user::{generate_token, validate_token, validate_username},
    models::{user::{UserCreateForm, UserLoginForm, UserDeleteForm, UpdateUserSettings}, Response, EMAIL_REGEX},
};
use actix_web::{get, post, web, HttpResponse, Responder, Scope, delete, patch};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, ModelTrait, Set};

pub fn get_routes() -> Scope {
    web::scope("/user")
        .service(info)
        .service(create)
        .service(login)
        .service(delete)
        .service(update)
}


#[get("/{user_id}")]
async fn info(path: web::Path<(String,)>, state: web::Data<Data>) -> Response<impl Responder> {
    let user_id = path.into_inner().0;
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
    if let Err(msg) = validate_username(&form.username) {
        return Ok(HttpResponse::BadRequest()
            .body(msg));
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
    let pass = match generate_token(&form.password) {
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

#[post("/login")]
async fn login(
    state: web::Data<Data>,
    form: web::Json<UserLoginForm>,
) -> Response<impl Responder> {
    let user = User::Entity::find()
        .filter(User::Column::Email.eq(form.email.to_owned()))
        .one(&state.database)
        .await?;
    
    if user.is_none() {
        return Ok(HttpResponse::BadRequest().body("Invalid Credentials"));
    }
    
    let user = user.unwrap();
    
    if !validate_token(&form.password, &user.token) {
        return Ok(HttpResponse::BadRequest().body("Invalid Credentials"));
    }

    Ok(HttpResponse::Ok().json(json!({
            "id": user.id,
            "email": user.email,
            "username": user.username,
            "token": user.token,
            "git_token": user.git_token,
            "created_at": user.created_at,
    })))
}

#[delete("")]
async fn delete(
    state: web::Data<Data>,
    form: web::Json<UserDeleteForm>,
) -> Response<impl Responder> {
    let user = User::Entity::find()
        .filter(User::Column::Id.eq(form.id.to_owned()))
        .one(&state.database)
        .await?;
    
    if user.is_none() {
        return Ok(HttpResponse::BadRequest().body("There is no such user"));
    }
    
    let user = user.unwrap();

    if !validate_token(&form.password, &user.token) {
        return Ok(HttpResponse::BadRequest().body("Invalid Credentials"));
    }

    user.delete(&state.database).await?;
    
    Ok(HttpResponse::Ok().body("User deleted"))
}

#[patch("")]
async fn update(
    state: web::Data<Data>,
    form: web::Json<UpdateUserSettings>,
) -> Response<impl Responder> {
    let user = User::Entity::find()
        .filter(User::Column::Email.eq(form.email.to_owned()))
        .one(&state.database)
        .await?;
    
    if user.is_none() {
        return Ok(HttpResponse::BadRequest().body("There is no such user"));
    }
    
    let user = user.unwrap();

    if !validate_token(&form.current_token, &user.token) {
        return Ok(HttpResponse::BadRequest().body("Invalid Credentials"));
    }
    let mut to_change = UpdateUserSettings {
        current_token: "".to_string(), // Don't change this

        email: None,
        username: None,
        git_token: None,
        new_token: None,
    };

    if let Some(new_password) = &form.new_token {
        to_change.new_token = match generate_token(&new_password) {
            Ok(val) => Some(val),
            Err(err) => {
                return Ok(HttpResponse::InternalServerError().body(err.to_string()));
            }
        };
    }

    if let Some(git_token) = &form.git_token {
        to_change.git_token = Some(git_token.to_owned());
    }

    if let Some(new_email) = &form.email {
        if !EMAIL_REGEX.is_match(&new_email) {
            return Ok(
                HttpResponse::BadRequest().body("Invalid email was provided")
            );
        }

        if User::Entity::find()
            .filter(User::Column::Email.eq(new_email.to_owned()))
            .one(&state.database)
            .await?
            .is_some()
        {
            return Ok(HttpResponse::Conflict().body("Email already exists"));
        }

        to_change.email = Some(new_email.to_string());
    }

    if let Some(new_username) = &form.username {
        if let Err(err) = validate_username(&new_username) {
            return Ok(HttpResponse::BadRequest().body(err));
        }

        if User::Entity::find()
            .filter(User::Column::Username.eq(new_username.to_owned()))
            .one(&state.database)
            .await?
            .is_some()
        {
            return Ok(HttpResponse::Conflict().body("Username already exists"));    
        }

        to_change.username = Some(new_username.to_string());
    }

    let mut update_model = User::ActiveModel {
        id: Set(user.id.to_owned()),
        ..Default::default()
    };

    // Update email if change validated
    if let Some(email) = &to_change.email {
        update_model.email = Set(email.to_owned());
    }

    // Update password if change validated
    if let Some(new_password) = to_change.new_token {
        update_model.token = Set(new_password);
    }

    // Update username if change validated
    if let Some(new_username) = to_change.username {
        update_model.username = Set(new_username);
    }

    // Update git token if change validated
    update_model.git_token = Set(to_change.git_token);

    // Perform all updates
    update_model.update(&state.database).await?;
    
    Ok(HttpResponse::Ok().json(json!({
            "id": user.id,
            "email": user.email,
            "username": user.username,
            "token": user.token,
            "git_token": user.git_token,
            "created_at": user.created_at,
    })))

}