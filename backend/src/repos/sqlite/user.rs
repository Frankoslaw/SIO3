use crate::models::user::*;
use crate::repos::user::*;
use async_trait::async_trait;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct SqliteUserRepoImpl {
    pub sqlite_pool: Arc<SqlitePool>,
}

// TODO: Add error handling
#[async_trait]
impl UserRepo for SqliteUserRepoImpl {
    async fn get_user(self: &Self, user_id: i32) -> UserModel {
        sqlx::query_as::<_, UserModel>("SELECT * FROM Users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&*self.sqlite_pool)
            .await
            .unwrap()
    }
}
