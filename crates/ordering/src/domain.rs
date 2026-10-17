use domain_core::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub product_type: String,
    pub product_id: Uuid,
    pub price_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    id: Uuid,
    customer_id: Uuid,
    status: OrderStatus,
    items: Vec<OrderItem>,
    total_amount_cents: i64,
}

impl AggregateRoot for Order {
    type Id = Uuid;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Order {
    pub fn new(customer_id: Uuid, items: Vec<OrderItem>) -> Self {
        let total_amount_cents = items.iter().map(|item| item.price_cents).sum();
        Self {
            id: Uuid::new_v4(),
            customer_id,
            status: OrderStatus::Pending,
            items,
            total_amount_cents,
        }
    }

    pub fn mark_processing(&mut self) {
        self.status = OrderStatus::Processing;
    }

    pub fn complete(&mut self) {
        self.status = OrderStatus::Completed;
    }

    pub fn fail(&mut self) {
        self.status = OrderStatus::Failed;
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn total_amount_cents(&self) -> i64 {
        self.total_amount_cents
    }
}
