use axum::{routing::get, Json, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load local .env variables if present
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/oxidetel".to_string());

    println!("🔌 Connecting to PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    println!("🔄 Running database migrations across all schemas...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Database migrations applied successfully.");

    // Build Axum router with initial health-check route
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 OxideTel BSS server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "system": "OxideTel BSS",
        "version": "0.1.0"
    }))
}
