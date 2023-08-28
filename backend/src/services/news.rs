use crate::models::{news::*, user::*};
use crate::repos::{news::*, user::*};
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait NewsService {
    async fn get_news(&self, competition_id: i32) -> Vec<NewsModelResponse>;
}

async fn filter_db_record(news: &NewsModel, author: &UserModel) -> NewsModelResponse {
    NewsModelResponse {
        id: news.id.to_owned(),
        author: author.username.to_owned(),
        title: news.title.to_owned(),
        content: news.content.to_owned(),
    }
}

pub struct NewsServiceImpl<A: NewsRepo, B: UserRepo> {
    pub news_repo: A,
    pub user_repo: B,
}

#[async_trait]
impl<A, B> NewsService for NewsServiceImpl<A, B>
where
    A: NewsRepo + Sync + Send,
    B: UserRepo + Sync + Send,
{
    // TODO: Figure out arbitrary self warning
    async fn get_news(self: &Self, competition_id: i32) -> Vec<NewsModelResponse> {
        let news_raw = self.news_repo.get_news(competition_id).await;
        let mut news: Vec<NewsModelResponse> = vec![];

        for new_raw in news_raw {
            let author = self.user_repo.get_user(new_raw.author_id).await;
            news.push(filter_db_record(&new_raw, &author).await);
        }

        news
    }
}
