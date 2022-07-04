#[derive(serde::Deserialize)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UserLoginForm {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UserDeleteForm {
    pub id: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_token: Option<String>,
    pub current_token: String,
}
