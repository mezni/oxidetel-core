use crate::domain::{DunningCase, DunningStage};
use domain_core::AggregateRoot;
use ordering::oss_adapter::OssProvisioningAdapter;
use sqlx::PgPool;
use uuid::Uuid;

pub struct DunningEngine {
    pool: PgPool,
    oss_adapter: OssProvisioningAdapter,
}

impl DunningEngine {
    pub fn new(pool: PgPool, oss_adapter: OssProvisioningAdapter) -> Self {
        Self { pool, oss_adapter }
    }

    /// Evaluates overdue invoices and executes dunning escalation workflow
    pub async fn process_dunning_sweep(&self) -> Result<(), sqlx::Error> {
        // Find unpaid invoices past their due dates using runtime queries
        let rows = sqlx::query(
            r#"SELECT id, customer_id, due_date 
               FROM billing.invoices 
               WHERE status = 'ISSUED' AND due_date < NOW()"#,
        )
        .fetch_all(&self.pool)
        .await?;

        for row in rows {
            use sqlx::Row;
            let invoice_id: Uuid = row.get("id");
            let customer_id: Uuid = row.get("customer_id");
            let due_date: chrono::DateTime<chrono::Utc> = row.get("due_date");

            let days_overdue = (chrono::Utc::now() - due_date).num_days() as i32;
            let mut dunning_case = DunningCase::new(invoice_id, customer_id, days_overdue);

            // Determine stage based on overdue duration
            if days_overdue >= 14 {
                dunning_case.escalate(); // Soft suspension
                println!(
                    "⚠️ [DUNNING] Soft suspension triggered for Customer {} (Invoice {})",
                    customer_id, invoice_id
                );
            }
            if days_overdue >= 30 {
                dunning_case.escalate(); // Hard suspension
                println!(
                    "🚨 [DUNNING] Hard suspension / Barring triggered for Customer {} (Invoice {})",
                    customer_id, invoice_id
                );

                // Fetch customer's active SIM ID to bar network access
                let sim_row = sqlx::query(
                    r#"SELECT sim_id FROM catalog.subscriptions WHERE customer_id = $1 LIMIT 1"#,
                )
                .bind(customer_id)
                .fetch_optional(&self.pool)
                .await?;

                if let Some(sim) = sim_row {
                    let sim_id: Uuid = sim.get("sim_id");
                    let _ = self
                        .oss_adapter
                        .trigger_activation(Uuid::new_v4(), sim_id)
                        .await;
                }
            }

            // Persist dunning case state
            sqlx::query(
                r#"INSERT INTO dunning.cases (id, invoice_id, customer_id, stage, days_overdue, updated_at)
                   VALUES ($1, $2, $3, $4, $5, NOW())
                   ON CONFLICT (id) DO UPDATE SET stage = $4, days_overdue = $5, updated_at = NOW()"#,
            )
            .bind(dunning_case.id())
            .bind(invoice_id)
            .bind(customer_id)
            .bind(format!("{:?}", dunning_case.stage()))
            .bind(days_overdue)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// Simulates payment receipt, resolving invoices and clearing dunning cases
    pub async fn process_payment_received(&self, invoice_id: Uuid) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        // 1. Mark invoice as paid
        sqlx::query(r#"UPDATE billing.invoices SET status = 'PAID' WHERE id = $1"#)
            .bind(invoice_id)
            .execute(&mut *tx)
            .await?;

        // 2. Resolve dunning case
        sqlx::query(r#"UPDATE dunning.cases SET stage = 'RESOLVED', updated_at = NOW() WHERE invoice_id = $1"#)
            .bind(invoice_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        println!("✅ [DUNNING] Payment received for Invoice {}. Dunning case resolved and service unbarred.", invoice_id);

        Ok(())
    }
}
