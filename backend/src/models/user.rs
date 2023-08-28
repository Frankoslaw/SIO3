use serde::{Deserialize, Serialize};

// TODO: Handle crypto on password
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    password: String,
    pub email: String,
    pub verified: bool,
}
