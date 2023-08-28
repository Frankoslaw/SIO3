use crate::models::user::*;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepo {
    async fn get_user(&self, user_id: i32) -> UserModel;
}
