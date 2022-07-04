#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod state;

use serde::{Deserialize, Serialize};
use surf::{Config, Url};

pub const BASE_API: &str = "http://45.134.10.233:8989";
pub const USER: &str = "/v1/user";

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    message: String
}

#[derive(Deserialize, Serialize, Debug)]
struct UserCreateResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub token: String,
}

#[tauri::command]
async fn user_create(
    state: tauri::State<'_, state::State>,
    username: String,
    email: String,
    password: String,
) -> Result<UserCreateResponse, ErrorResponse> {
    let body = serde_json::json!({
        "username": username,
        "email": email,
        "password": password
    });
    let mut res = state
        .client
        .post(USER)
        .body(http_types::Body::from_json(&body).unwrap()).await.expect("user create has errored");
    if res.status() != http_types::StatusCode::Ok {
        let msg = res.body_string().await.unwrap();
        return Err(ErrorResponse {
            message: format!("Status not OK. Received: {}. Message: {}", res.status(), msg)
        })
    }
    println!("{:?}", res);

    let response: UserCreateResponse = res.body_json().await.unwrap();
    // Ok(UserCreateResponse {
    //     id: response.id,
    //     username: response.username,
    //     email: response.email,
    //     created_at: response.created_at,
    //     token: response.token,
    // })
    Ok(UserCreateResponse {
        id: "".to_string(),
        username: "".to_string(),
        email: "".to_string(),
        created_at: "".to_string(),
        token: "".to_string()
    })
}

#[tauri::command]
async fn user_get() {}

#[tauri::command]
async fn user_update() {}

#[tauri::command]
async fn user_delete() {}

#[tauri::command]
async fn user_login() {}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(state::State {
            client: Config::new()
                .set_base_url(Url::parse(BASE_API.into()).expect("couldnt parse url"))
                .try_into()
                .unwrap(),
        })
        .invoke_handler(tauri::generate_handler![
            user_create,
            user_get,
            user_update,
            user_delete,
            user_login
        ])
        // .menu(tauri::Menu::os_default(&context.package_info().name))
        .run(context)
        .expect("error while running tauri application");
}
