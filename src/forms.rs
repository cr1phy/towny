use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UserForm {
    pub email: String,
    pub password: String,
    pub name: String,
}
