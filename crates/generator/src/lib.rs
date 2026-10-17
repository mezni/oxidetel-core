use fake::faker::internet::raw::SafeEmail;
use fake::faker::name::raw::Name;
use fake::Fake;
use rand::Rng;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TelephonyDataGenerator {
    pool: PgPool,
}

impl TelephonyDataGenerator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Seeds mock customers, SIMs, and rate plans across schemas using runtime queries
    pub async fn seed_base_system(&self, count: usize) -> Result<Vec<Uuid>, sqlx::Error> {
        let mut customer_ids = Vec::new();
        let mut rng = rand::thread_rng();

        // 1. Seed a default Rate Plan if not already present
        let rate_plan_id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO catalog.rate_plans 
               (id, name, code, monthly_fee_cents, included_data_bytes, overage_rate_per_mb_cents)
               VALUES ($1, '5G Unlimited 10GB Cap', 'PLAN_5G_10GB', 4999, 10737418240, 5)
               ON CONFLICT (code) DO NOTHING"#,
        )
        .bind(rate_plan_id)
        .execute(&self.pool)
        .await?;

        for _ in 0..count {
            let customer_id = Uuid::new_v4();
            let name: String = Name(fake::locales::EN).fake();
            let email: String = SafeEmail(fake::locales::EN).fake();

            // 2. Seed CRM Schema
            sqlx::query(
                r#"INSERT INTO crm.customers (id, name, email, status) VALUES ($1, $2, $3, 'ACTIVE')"#,
            )
            .bind(customer_id)
            .bind(&name)
            .bind(&email)
            .execute(&self.pool)
            .await?;

            // 3. Seed Catalog / Inventory Schema (SIM Card)
            let iccid: String = format!("890141032111{:08}", rng.gen_range(10000000..99999999));
            let imsi: String = format!("310260{:09}", rng.gen_range(100000000..999999999));
            let msisdn: String = format!("+1555{:07}", rng.gen_range(1000000..9999999));

            sqlx::query(
                r#"INSERT INTO catalog.sim_cards (id, customer_id, iccid, imsi, msisdn, status) 
                   VALUES ($1, $2, $3, $4, $5, 'ACTIVATED')"#,
            )
            .bind(Uuid::new_v4())
            .bind(customer_id)
            .bind(&iccid)
            .bind(&imsi)
            .bind(&msisdn)
            .execute(&self.pool)
            .await?;

            customer_ids.push(customer_id);
        }

        Ok(customer_ids)
    }

    /// Continuously simulates data usage streams (CDRs) into the Rating schema
    pub async fn stream_cdrs(&self, customer_ids: &[Uuid], duration_secs: u64) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));
        let start = std::time::Instant::now();
        let mut rng = rand::thread_rng();

        println!("⚡ [GENERATOR] Starting real-time CDR Data stream...");

        while start.elapsed().as_secs() < duration_secs {
            interval.tick().await;

            if customer_ids.is_empty() {
                break;
            }

            let customer_id = customer_ids[rng.gen_range(0..customer_ids.len())];
            let megabytes_used: i32 = rng.gen_range(10..200);
            let bytes_used = (megabytes_used as i64) * 1_024 * 1_024;

            let res = sqlx::query(
                r#"INSERT INTO rating.cdrs (id, customer_id, usage_type, bytes_used, created_at)
                   VALUES ($1, $2, 'DATA', $3, NOW())"#,
            )
            .bind(Uuid::new_v4())
            .bind(customer_id)
            .bind(bytes_used)
            .execute(&self.pool)
            .await;

            if let Err(e) = res {
                eprintln!("Failed to stream CDR: {:?}", e);
            }
        }
        println!("✅ [GENERATOR] CDR Data Streaming completed.");
    }
}
