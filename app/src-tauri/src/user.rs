use crate::BASE_API;
use ureq;

const USER: &str = "/user";

#[tauri::command]
pub async fn create() {}

#[tauri::command]
pub async fn get() {}

#[tauri::command]
pub async fn update() {}

#[tauri::command]
pub async fn delete() {}

#[tauri::command]
pub async fn login() {}