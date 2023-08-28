use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn configure() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var needs to be set");
    configure_with_db_url(&db_url).await
}

pub async fn configure_with_db_url(db_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Unable to connect to Postgresql");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Unable to migrate DB");

    pool
}
