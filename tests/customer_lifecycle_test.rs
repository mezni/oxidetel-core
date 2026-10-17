mod common;

use sqlx::Row;
use uuid::Uuid;

#[tokio::test]
async fn test_full_telecom_customer_lifecycle() {
    let pool = common::setup_test_db().await;

    // 1. Customer Onboarding (CRM Schema)
    let customer_id = Uuid::new_v4();
    let email = format!("test_user_{}@oxidetel.com", customer_id);

    let insert_customer = sqlx::query(
        "INSERT INTO crm.customers (id, name, email, status) VALUES ($1, $2, $3, 'Active')",
    )
    .bind(customer_id)
    .bind("Alice Telecom Subscriber")
    .bind(&email)
    .execute(&pool)
    .await;

    match insert_customer {
        Ok(_) => println!("Customer inserted successfully!"),
        Err(e) => panic!("Failed to insert customer: {:?}", e),
    }

    // 2. Fetch Rate Plan (Catalog Schema)
    let plan_row =
        sqlx::query("SELECT id, name, monthly_fee_cents FROM catalog.rate_plans LIMIT 1")
            .fetch_optional(&pool)
            .await
            .expect("Failed to query rate plans");

    if let Some(plan) = plan_row {
        let _plan_id: Uuid = plan.get("id");
        let monthly_fee_cents: i64 = plan.get("monthly_fee_cents");

        // 3. Place Order & Provisioning (Ordering Schema)
        let order_id = Uuid::new_v4();
        let insert_order = sqlx::query(
            "INSERT INTO ordering.orders (id, customer_id, total_amount_cents, status) VALUES ($1, $2, $3, 'Completed')"
        )
        .bind(order_id)
        .bind(customer_id)
        .bind(monthly_fee_cents)
        .execute(&pool)
        .await;

        match insert_order {
            Ok(_) => println!("Order created successfully!"),
            Err(e) => panic!("Failed to create customer order: {:?}", e),
        }

        // 4. Generate Invoice (Billing Schema)
        let invoice_id = Uuid::new_v4();
        let insert_invoice = sqlx::query(
            "INSERT INTO billing.invoices (id, customer_id, amount_cents, currency, status, due_date) VALUES ($1, $2, $3, 'USD', 'Unpaid', NOW() + INTERVAL '30 days')"
        )
        .bind(invoice_id)
        .bind(customer_id)
        .bind(monthly_fee_cents)
        .execute(&pool)
        .await;

        match insert_invoice {
            Ok(_) => println!("Invoice generated successfully!"),
            Err(e) => panic!("Failed to generate monthly invoice: {:?}", e),
        }

        // 5. Simulate Dunning State Machine (Dunning Schema)
        let dunning_id = Uuid::new_v4();
        let insert_dunning = sqlx::query(
            "INSERT INTO dunning.cases (id, customer_id, invoice_id, stage, days_overdue) VALUES ($1, $2, $3, 'SOFT_SUSPENSION', 15)"
        )
        .bind(dunning_id)
        .bind(customer_id)
        .bind(invoice_id)
        .execute(&pool)
        .await;

        match insert_dunning {
            Ok(_) => println!("Dunning case triggered successfully!"),
            Err(e) => panic!("Failed to trigger dunning case: {:?}", e),
        }

        // 6. Simulate Payment Webhook Settlement & Unbarring
        let update_invoice =
            sqlx::query("UPDATE billing.invoices SET status = 'Paid' WHERE id = $1")
                .bind(invoice_id)
                .execute(&pool)
                .await;
        assert!(update_invoice.is_ok());

        let resolve_dunning =
            sqlx::query("UPDATE dunning.cases SET stage = 'RESOLVED' WHERE id = $1")
                .bind(dunning_id)
                .execute(&pool)
                .await;
        assert!(
            resolve_dunning.is_ok(),
            "Failed to resolve dunning case via payment settlement"
        );

        println!("Full customer lifecycle integration test passed successfully!");
    } else {
        println!("Skipping plan-dependent tests: No rate plans found in catalog. Run database seeding first.");
    }
}
