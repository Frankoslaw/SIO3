use crate::models::news::*;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait NewsRepo {
    async fn get_news(&self, competition_id: i32) -> Vec<NewsModel>;
}
