use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/customers", post(register_customer))
        .route("/api/catalog/plans", get(list_rate_plans))
        .route("/api/orders", post(place_order))
        .route("/api/invoices/:customer_id", get(get_customer_invoices))
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
