mod controllers;
mod infra;
mod models;
mod repos;
mod services;

use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::SqlitePool;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// TODO: This error error handler
#[tokio::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = dotenv::dotenv() {
        print!("Not applying .env : {:?}", e);
    }

    // let _pg_pool = Arc::new(infra::postgresql::configure().await);
    let sqlite_pool = Arc::new(infra::sqlite::configure().await);

    let port = env::var("PORT").expect("PORT env var must be set");
    println!("Actix API at 127.0.0.1:{} is ready!", port);

    HttpServer::new(move || {
        App::new().configure(|cfg| configure_features(sqlite_pool.clone(), cfg))
    })
    .bind(("127.0.0.1", port.parse::<u16>().unwrap()))?
    .run()
    .await
}

// TODO: Replace db adapter with adapter pattern
fn configure_features(sqlite_pool: Arc<SqlitePool>, cfg: &mut web::ServiceConfig) {
    configure_news(sqlite_pool.clone(), cfg);
}

fn configure_news(sqlite_pool: Arc<SqlitePool>, cfg: &mut web::ServiceConfig) {
    use crate::controllers::rest_news;
    use crate::repos::sqlite::news::SqliteNewsRepoImpl;
    use crate::repos::sqlite::user::SqliteUserRepoImpl;
    use crate::services::news::NewsServiceImpl;

    let service = NewsServiceImpl {
        news_repo: SqliteNewsRepoImpl {
            sqlite_pool: sqlite_pool.clone(),
        },
        user_repo: SqliteUserRepoImpl {
            sqlite_pool: sqlite_pool.clone(),
        },
    };

    rest_news::configure(web::Data::new(service), cfg);
}
