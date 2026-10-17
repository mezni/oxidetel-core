use crate::domain::{Invoice, InvoiceItem};
use domain_core::AggregateRoot;
use sqlx::PgPool;
use uuid::Uuid;

pub struct BillingEngine {
    pool: PgPool,
}

impl BillingEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Generates a monthly invoice for a subscriber by combining plan fees and data overage usage
    pub async fn generate_customer_invoice(
        &self,
        customer_id: Uuid,
    ) -> Result<Invoice, sqlx::Error> {
        // 1. Fetch customer's active subscription and rate plan details
        let row = sqlx::query!(
            r#"SELECT rp.name as plan_name, rp.monthly_fee_cents, rp.included_data_bytes, rp.overage_rate_per_mb_cents 
               FROM catalog.subscriptions s
               JOIN catalog.rate_plans rp ON s.rate_plan_id = rp.id
               WHERE s.customer_id = $1 AND s.status = 'ACTIVE'
               LIMIT 1"#,
            customer_id
        )
        .fetch_optional(&self.pool)
        .await?;

        let mut items = Vec::new();

        if let Some(plan) = row {
            // Add monthly recurring plan fee
            items.push(InvoiceItem {
                id: Uuid::new_v4(),
                description: format!("Monthly Subscription: {}", plan.plan_name),
                amount_cents: plan.monthly_fee_cents,
            });

            // 2. Aggregate total data used from rating.cdrs for the current billing cycle
            let usage_row = sqlx::query!(
                r#"SELECT COALESCE(SUM(bytes_used), 0)::BIGINT as total_bytes 
                   FROM rating.cdrs 
                   WHERE customer_id = $1 AND usage_type = 'DATA' 
                   AND created_at >= NOW() - INTERVAL '30 days'"#,
                customer_id
            )
            .fetch_one(&self.pool)
            .await?;

            let total_bytes: i64 = usage_row.total_bytes.unwrap_or(0);

            // 3. Calculate overage charges if data exceeds the plan allowance
            if total_bytes > plan.included_data_bytes {
                let excess_bytes = total_bytes - plan.included_data_bytes;
                let excess_mb = (excess_bytes as f64 / 1_048_576.0).ceil() as i64;
                let overage_cents = excess_mb * plan.overage_rate_per_mb_cents;

                if overage_cents > 0 {
                    items.push(InvoiceItem {
                        id: Uuid::new_v4(),
                        description: format!("Data Overage: {} MB exceeded", excess_mb),
                        amount_cents: overage_cents,
                    });
                }
            }
        } else {
            // Fallback base charge if subscription is unlinked
            items.push(InvoiceItem {
                id: Uuid::new_v4(),
                description: "Standard Monthly Service Fee".to_string(),
                amount_cents: 4999,
            });
        }

        // Create the invoice aggregate (Net 30 terms)
        let invoice = Invoice::new(customer_id, items, 30);

        // Persist invoice and line items transactionally
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"INSERT INTO billing.invoices (id, customer_id, amount_cents, currency, status, due_date, created_at)
               VALUES ($1, $2, $3, $4, 'ISSUED', $5, NOW())"#,
        )
        .bind(invoice.id())
        .bind(customer_id)
        .bind(invoice.amount_cents())
        .bind("USD")
        .bind(chrono::Utc::now() + chrono::Duration::days(30))
        .execute(&mut *tx)
        .await?;

        for item in invoice.items() {
            sqlx::query(
                r#"INSERT INTO billing.invoice_items (id, invoice_id, description, amount_cents)
                   VALUES ($1, $2, $3, $4)"#,
            )
            .bind(item.id)
            .bind(invoice.id())
            .bind(&item.description)
            .bind(item.amount_cents)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        println!(
            "💳 [BILLING] Generated Invoice {} for Customer {} (Total: ${:.2})",
            invoice.id(),
            customer_id,
            invoice.amount_cents() as f64 / 100.0
        );

        Ok(invoice)
    }
}
