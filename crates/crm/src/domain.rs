use domain_core::AggregateRoot;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerStatus {
    Active,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    id: Uuid,
    name: String,
    email: String,
    status: CustomerStatus,
}

impl AggregateRoot for Customer {
    type Id = Uuid;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Customer {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            status: CustomerStatus::Active,
        }
    }

    pub fn suspend(&mut self) {
        self.status = CustomerStatus::Suspended;
    }

    pub fn activate(&mut self) {
        self.status = CustomerStatus::Active;
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
