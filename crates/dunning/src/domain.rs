use domain_core::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DunningStage {
    PastDue,
    SoftSuspension,
    HardSuspension,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningCase {
    id: Uuid,
    invoice_id: Uuid,
    customer_id: Uuid,
    stage: DunningStage,
    days_overdue: i32,
}

impl AggregateRoot for DunningCase {
    type Id = Uuid;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl DunningCase {
    pub fn new(invoice_id: Uuid, customer_id: Uuid, days_overdue: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            invoice_id,
            customer_id,
            stage: DunningStage::PastDue,
            days_overdue,
        }
    }

    pub fn escalate(&mut self) {
        self.stage = match self.stage {
            DunningStage::PastDue => DunningStage::SoftSuspension,
            DunningStage::SoftSuspension => DunningStage::HardSuspension,
            DunningStage::HardSuspension => DunningStage::HardSuspension,
            DunningStage::Resolved => DunningStage::Resolved,
        };
    }

    pub fn resolve(&mut self) {
        self.stage = DunningStage::Resolved;
    }

    pub fn stage(&self) -> &DunningStage {
        &self.stage
    }
}
