use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use tower_http::trace::TraceLayer;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/customers", post(register_customer))
        .route("/api/catalog/plans", get(list_rate_plans))
        .route("/api/orders", post(place_order))
        .route("/api/invoices/:customer_id", get(get_customer_invoices))
        .route("/api/webhooks/payment", post(handle_payment_webhook))
        .layer(TraceLayer::new_for_http()) // <-- Adds structured request tracing
        .with_state(pool)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "system": "OxideTel BSS",
        "version": "1.0.0"
    }))
}

// --- DTOs & Models ---

#[derive(Deserialize)]
pub struct RegisterCustomerDto {
    name: String,
    email: String,
}

#[derive(Serialize, FromRow)]
pub struct RatePlanResponse {
    id: Uuid,
    name: String,
    code: String,
    monthly_fee_cents: i64,
    included_data_bytes: i64,
}

#[derive(Deserialize)]
pub struct PlaceOrderDto {
    customer_id: Uuid,
    rate_plan_id: Uuid,
}

#[derive(Serialize, FromRow)]
pub struct InvoiceResponse {
    id: Uuid,
    amount_cents: i64,
    currency: String,
    status: String,
    due_date: chrono::DateTime<chrono::Utc>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

// --- Handlers ---

async fn register_customer(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterCustomerDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let customer_id = Uuid::new_v4();

    let res = sqlx::query(
        r#"INSERT INTO crm.customers (id, name, email, status) VALUES ($1, $2, $3, 'ACTIVE')"#,
    )
    .bind(customer_id)
    .bind(&payload.name)
    .bind(&payload.email)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({
                "customer_id": customer_id,
                "name": payload.name,
                "email": payload.email,
                "status": "ACTIVE"
            })),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_rate_plans(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<RatePlanResponse>>, StatusCode> {
    let plans = sqlx::query_as::<_, RatePlanResponse>(
        r#"SELECT id, name, code, monthly_fee_cents, included_data_bytes 
           FROM catalog.rate_plans WHERE is_active = TRUE"#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(plans))
}

async fn place_order(
    State(pool): State<PgPool>,
    Json(payload): Json<PlaceOrderDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let order_id = Uuid::new_v4();

    // Fetch plan fee using runtime query and row extraction
    let plan_row = sqlx::query(r#"SELECT monthly_fee_cents FROM catalog.rate_plans WHERE id = $1"#)
        .bind(payload.rate_plan_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    use sqlx::Row;
    let monthly_fee_cents: i64 = plan_row.get("monthly_fee_cents");

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"INSERT INTO ordering.orders (id, customer_id, status, total_amount_cents, created_at)
           VALUES ($1, $2, 'COMPLETED', $3, NOW())"#,
    )
    .bind(order_id)
    .bind(payload.customer_id)
    .bind(monthly_fee_cents)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "order_id": order_id,
            "customer_id": payload.customer_id,
            "status": "COMPLETED",
            "total_amount_cents": monthly_fee_cents
        })),
    ))
}

async fn get_customer_invoices(
    State(pool): State<PgPool>,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<Vec<InvoiceResponse>>, StatusCode> {
    let invoices = sqlx::query_as::<_, InvoiceResponse>(
        r#"SELECT id, amount_cents, currency, status, due_date, created_at 
           FROM billing.invoices WHERE customer_id = $1"#,
    )
    .bind(customer_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(invoices))
}

// --- Payment Webhook DTO & Handler ---

#[derive(Deserialize)]
pub struct PaymentWebhookDto {
    invoice_id: Uuid,
    customer_id: Uuid,
    amount_cents: i64,
    gateway_reference: String,
}

async fn handle_payment_webhook(
    State(pool): State<PgPool>,
    Json(payload): Json<PaymentWebhookDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let payment_id = Uuid::new_v4();
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 1. Record the payment transaction
    sqlx::query(
        r#"INSERT INTO billing.payments (id, invoice_id, customer_id, amount_cents, gateway_reference, status, created_at)
           VALUES ($1, $2, $3, $4, $5, 'SUCCESS', NOW())"#,
    )
    .bind(payment_id)
    .bind(payload.invoice_id)
    .bind(payload.customer_id)
    .bind(payload.amount_cents)
    .bind(&payload.gateway_reference)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Mark the invoice as PAID
    sqlx::query(r#"UPDATE billing.invoices SET status = 'PAID' WHERE id = $1"#)
        .bind(payload.invoice_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Resolve any active dunning cases
    sqlx::query(
        r#"UPDATE dunning.cases SET stage = 'RESOLVED', updated_at = NOW() WHERE invoice_id = $1"#,
    )
    .bind(payload.invoice_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 4. Unbar / Reactivate subscriber service at the OSS network layer
    let sim_row =
        sqlx::query(r#"SELECT sim_id FROM catalog.subscriptions WHERE customer_id = $1 LIMIT 1"#)
            .bind(payload.customer_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(sim) = sim_row {
        use sqlx::Row;
        let sim_id: Uuid = sim.get("sim_id");
        println!(
            "📡 [OSS ADAPTER] Sending UNBAR / RESTORE command for SIM ID: {}",
            sim_id
        );

        // Update subscription status back to ACTIVE
        sqlx::query(r#"UPDATE catalog.subscriptions SET status = 'ACTIVE' WHERE customer_id = $1"#)
            .bind(payload.customer_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!(
        "✅ [WEBHOOK] Payment of ${:.2} processed successfully for Invoice {}.",
        payload.amount_cents as f64 / 100.0,
        payload.invoice_id
    );

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "payment_id": payment_id,
            "invoice_id": payload.invoice_id,
            "message": "Payment processed, dunning resolved, and service restored."
        })),
    ))
}
