use domain_core::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlan {
    id: Uuid,
    name: String,
    code: String,
    monthly_fee_cents: i64,
    included_data_bytes: i64,
    overage_rate_per_mb_cents: i64,
    is_active: bool,
}

impl AggregateRoot for RatePlan {
    type Id = Uuid;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl RatePlan {
    pub fn new(
        name: String,
        code: String,
        monthly_fee_cents: i64,
        included_data_bytes: i64,
        overage_rate_per_mb_cents: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            code,
            monthly_fee_cents,
            included_data_bytes,
            overage_rate_per_mb_cents,
            is_active: true,
        }
    }

    /// Calculates overage cost in cents given total usage in bytes
    pub fn calculate_overage(&self, total_used_bytes: i64) -> i64 {
        if total_used_bytes <= self.included_data_bytes {
            0
        } else {
            let excess_bytes = total_used_bytes - self.included_data_bytes;
            let excess_mb = (excess_bytes as f64 / 1_048_576.0).ceil() as i64;
            excess_mb * self.overage_rate_per_mb_cents
        }
    }
}
