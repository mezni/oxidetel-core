use async_trait::async_trait;
use serde::Serialize;

/// Every DDD Aggregate Root must implement this trait to expose its typed identity.
pub trait AggregateRoot {
    type Id: Copy + Eq + Send + Sync;
    fn id(&self) -> Self::Id;
}

/// Marker trait for inter-domain events.
pub trait DomainEvent: Send + Sync + Serialize {
    fn event_type(&self) -> &'static str;
}

/// Generic repository trait for persistence boundaries.
#[async_trait]
pub trait Repository<A: AggregateRoot>: Send + Sync {
    type Error;
    async fn save(&self, aggregate: &A) -> Result<(), Self::Error>;
    async fn find_by_id(&self, id: A::Id) -> Result<Option<A>, Self::Error>;
}
