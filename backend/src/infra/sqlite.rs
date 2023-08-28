use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use std::env;

// TODO: Unify implementation with postgress
pub async fn configure() -> SqlitePool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var needs to be set");
    configure_with_db_url(&db_url).await
}

pub async fn configure_with_db_url(db_url: &str) -> SqlitePool {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        Sqlite::create_database(db_url).await.unwrap();
    }

    let pool = SqlitePoolOptions::new()
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
