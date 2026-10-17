use axum::{routing::get, Json, Router};
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

    // --- End-to-End Simulation Execution ---
    println!("\n🚀 === STARTING OXIDETEL BSS E2E SIMULATION ===");

    let generator = TelephonyDataGenerator::new(pool.clone());
    let billing_engine = BillingEngine::new(pool.clone());
    let oss_adapter = OssProvisioningAdapter::new("http://mock-oss-core.local".to_string());
    let dunning_engine = DunningEngine::new(pool.clone(), oss_adapter);

    // 1. Acquire Subscribers
    println!("👤 Step 1: Acquiring test subscribers and seeding inventory...");
    let customer_ids = generator.seed_base_system(3).await?;
    println!("✅ Seeded {} subscribers.", customer_ids.len());

    // 2. Stream Data Usage (CDRs)
    println!("📊 Step 2: Simulating real-time network data consumption...");
    generator.stream_cdrs(&customer_ids, 3).await;

    // 3. Generate Invoices
    println!("💳 Step 3: Running billing engine to generate monthly invoices...");
    for customer_id in &customer_ids {
        let _invoice = billing_engine
            .generate_customer_invoice(*customer_id)
            .await?;
    }

    // 4. Run Dunning Sweep
    println!("⚖️ Step 4: Executing collections & dunning compliance check...");
    dunning_engine.process_dunning_sweep().await?;

    println!("🎉 === OXIDETEL BSS E2E SIMULATION COMPLETED SUCCESSFULLY ===\n");

    // Start Axum web server
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 OxideTel BSS server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "system": "OxideTel BSS",
        "version": "1.0.0"
    }))
}
