use sqlx::PgPool;
use tokio::time::{self, Duration};
use tracing::{error, info, warn};

pub async fn start_dunning_worker(pool: PgPool) {
    // Run dunning sweep every 30 seconds for demonstration purposes
    let mut interval = time::interval(Duration::from_secs(30));

    info!("Background Dunning & Collections Worker started.");

    loop {
        interval.tick().await;
        info!("Executing scheduled dunning sweep...");

        if let Err(e) = run_dunning_sweep(&pool).await {
            error!("Error executing dunning sweep: {:?}", e);
        }
    }
}

async fn run_dunning_sweep(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 1. Find unpaid invoices past their due date that do not have active dunning cases yet
    let overdue_invoices = sqlx::query!(
        r#"
        SELECT i.id as invoice_id, i.customer_id, i.due_date
        FROM billing.invoices i
        LEFT JOIN dunning.cases d ON i.id = d.invoice_id
        WHERE i.status = 'Unpaid' 
          AND i.due_date < NOW()
          AND d.id IS NULL
        "#
    )
    .fetch_all(pool)
    .await?;

    for inv in overdue_invoices {
        let invoice_id = match inv.invoice_id {
            Some(id) => id,
            None => continue,
        };
        let customer_id = match inv.customer_id {
            Some(id) => id,
            None => continue,
        };

        let dunning_id = uuid::Uuid::new_v4();
        warn!(
            "Invoice {} for customer {} is overdue! Initiating Soft Suspension dunning case.",
            invoice_id, customer_id
        );

        // Create dunning case in 'SOFT_SUSPENSION' stage
        sqlx::query!(
            "INSERT INTO dunning.cases (id, invoice_id, customer_id, stage, days_overdue) VALUES ($1, $2, $3, 'SOFT_SUSPENSION', 1)",
            dunning_id,
            invoice_id,
            customer_id
        )
        .execute(pool)
        .await?;

        // Update CRM status to reflect soft suspension
        sqlx::query!(
            "UPDATE crm.customers SET status = 'Suspended' WHERE id = $1",
            customer_id
        )
        .execute(pool)
        .await?;
    }

    // 2. Escalate old soft-suspended cases to hard suspension after threshold
    let escalate_cases = sqlx::query!(
        r#"
        UPDATE dunning.cases
        SET stage = 'HARD_SUSPENSION', days_overdue = days_overdue + 1
        WHERE stage = 'SOFT_SUSPENSION' AND updated_at < NOW() - INTERVAL '1 minute'
        RETURNING id, customer_id
        "#
    )
    .fetch_all(pool)
    .await?;

    for case in escalate_cases {
        let case_id = case.id;
        let customer_id = case.customer_id;

        warn!(
            "Dunning case {} for customer {} escalated to HARD_SUSPENSION (service fully barred).",
            case_id, customer_id
        );
    }

    info!("Dunning sweep completed successfully.");
    Ok(())
}
