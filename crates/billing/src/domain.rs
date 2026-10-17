use chrono::{DateTime, Utc};
use domain_core::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvoiceStatus {
    Issued,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: Uuid,
    pub description: String,
    pub amount_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    id: Uuid,
    customer_id: Uuid,
    amount_cents: i64,
    currency: String,
    status: InvoiceStatus,
    items: Vec<InvoiceItem>,
    due_date: DateTime<Utc>,
}

impl AggregateRoot for Invoice {
    type Id = Uuid;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Invoice {
    pub fn new(customer_id: Uuid, items: Vec<InvoiceItem>, net_terms_days: i64) -> Self {
        let amount_cents = items.iter().map(|item| item.amount_cents).sum();
        Self {
            id: Uuid::new_v4(),
            customer_id,
            amount_cents,
            currency: "USD".to_string(),
            status: InvoiceStatus::Issued,
            items,
            due_date: Utc::now() + chrono::Duration::days(net_terms_days),
        }
    }

    pub fn items(&self) -> &[InvoiceItem] {
        &self.items
    }

    pub fn mark_paid(&mut self) {
        self.status = InvoiceStatus::Paid;
    }

    pub fn mark_overdue(&mut self) {
        self.status = InvoiceStatus::Overdue;
    }

    pub fn status(&self) -> &InvoiceStatus {
        &self.status
    }

    pub fn amount_cents(&self) -> i64 {
        self.amount_cents
    }
}
