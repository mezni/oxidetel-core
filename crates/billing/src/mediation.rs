use sqlx::PgPool;
use tracing::{info, error};
use uuid::Uuid;

pub async fn process_cdr_mediation_batch(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Starting CDR mediation and rating batch sweep...");

    // 1. Fetch unrated CDRs from the rating schema
    let unrated_cdrs = sqlx::query!(
        r#"
        SELECT id, customer_id, duration_seconds, data_megabytes, usage_type
        FROM rating.cdrs
        WHERE rated = FALSE
        LIMIT 100
        "
    )
    .fetch_all(pool)
    .await?;

    if unrated_cdrs.is_empty() {
        info!("No unrated CDRs found.");
        return Ok(issons = ());
    }

    for cdr in unrated_cdrs {
        let cdr_id = cdr.id;
        let customer_id = match cdr.customer_id {
            Some(id) => id,
            None => continue,
        };
        let data_mb = cdr.data_megabytes.unwrap_or(0);

        // 2. Fetch customer's active rate plan to check overage rates
        let plan_info = sqlx::query!(
            r#"
            SELECT rp.overage_rate_per_mb_cents
            FROM catalog.subscriptions s
            JOIN catalog.rate_plans rp ON s.rate_plan_id = rp.id
            WHERE s.customer_id = $1 AND s.status = 'Active'
            LIMIT 1
            "#,
            customer_id
        )
        .fetch_optional(pool)
        .await?;

        let overage_rate_cents = if let Some(plan) = plan_info {
            plan.overage_rate_per_mb_cents
        } else {
            10 // Default fallback overage rate: 10 cents per MB
        };

        let calculated_charge_cents = data_mb * overage_rate_cents;

        if calculated_charge_cents > 0 {
            // 3. Find current open invoice for the customer
            let invoice = sqlx::query!(
                r#"
                SELECT id FROM billing.invoices
                WHERE customer_id = $1 AND status = 'Unpaid'
                ORDER BY due_date ASC
                LIMIT 1
                "#,
                customer_id
            )
            .fetch_optional(pool)
            .await?;

            if let Some(inv) = invoice {
                let invoice_id = inv.id;
                let item_id = Uuid::new_v4();

                // Add overage line item to invoice
                sqlx::query!(
                    r#"
                    INSERT INTO billing.invoice_items (id, invoice_id, description, amount_cents)
                    VALUES ($1, $2, $3, $4)
                    "#,
                    item_id,
                    invoice_id,
                    format!("Data overage charge: {} MB @ {}¢/MB", data_mb, overage_rate_cents),
                    calculated_charge_cents
                )
                .execute(pool)
                .await?;

                // Update invoice total amount
                sqlx::query!(
                    r#"
                    UPDATE billing.invoices
                    SET amount_cents = amount_cents + $1
                    WHERE id = $2
                    "#,
                    calculated_charge_cents,
                    invoice_id
                )
                .execute(pool)
                .await?;
            }
        }

        // 4. Mark CDR as rated
        sqlx::query!(
            "UPDATE rating.cdrs SET rated = TRUE WHERE id = $1",
            cdr_id
        )
        .execute(pool)
        .await?;
    }

    info!("CDR mediation batch completed successfully.");
    Ok(())
}
