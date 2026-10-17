mod api;

use dotenvy::dotenv;
use metrics_exporter_prometheus::PrometheusBuilder;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 1. Initialize Structured Logging & Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "oxidetel_bss=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Initializing OxideTel BSS Engine...");

    // 2. Initialize Prometheus Metrics Exporter
    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/oxidetel".to_string());

    tracing::info!("Connecting to PostgreSQL database pool...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    tracing::info!("Executing database multi-schema migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Database migrations applied successfully.");

    // 3. Build Axum Router & attach metrics endpoint
    let app = api::create_router(pool).route(
        "/metrics",
        axum::routing::get(move || async move { recorder_handle.render() }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("OxideTel BSS server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
