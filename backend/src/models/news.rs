use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct NewsModel {
    pub id: i32,
    pub competition_id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewsModelResponse {
    pub id: i32,
    pub author: String,
    pub title: String,
    pub content: String,
}
