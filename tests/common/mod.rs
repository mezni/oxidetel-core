use sqlx::PgPool;
use std::env;

pub async fn setup_test_db() -> PgPool {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/oxidetel".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test db");

    // Run migrations to ensure schemas are up to date
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
