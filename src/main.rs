mod api;

use billing::engine::BillingEngine;
use dotenvy::dotenv;
use dunning::engine::DunningEngine;
use generator::TelephonyDataGenerator;
use ordering::oss_adapter::OssProvisioningAdapter;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/oxidetel".to_string());

    println!("🔌 Connecting to PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    println!("🔄 Running multi-schema database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("✅ Database migrations applied successfully.");

    // --- Optional E2E Simulation Run on Startup ---
    println!("\n🚀 === EXECUTING INITIAL TELECOM BOOTSTRAP SEED ===");
    let generator = TelephonyDataGenerator::new(pool.clone());
    let _customer_ids = generator.seed_base_system(2).await?;
    println!("✅ Initial subscribers seeded.");

    // Build Axum router with REST API endpoints
    let app = api::create_router(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 OxideTel BSS REST API running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
