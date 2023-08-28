use crate::models::news::*;
use crate::repos::news::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct SqliteNewsRepoImpl {
    pub sqlite_pool: Arc<SqlitePool>,
}

// TODO: Add error handling
#[async_trait]
impl NewsRepo for SqliteNewsRepoImpl {
    async fn get_news(self: &Self, competition_id: i32) -> Vec<NewsModel> {
        sqlx::query_as::<_, NewsModel>("SELECT * FROM News WHERE competition_id = ?")
            .bind(competition_id)
            .fetch_all(&*self.sqlite_pool)
            .await
            .unwrap()
    }
}
