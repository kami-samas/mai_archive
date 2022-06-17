
#[derive(serde::Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String
}