use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::{ContactId, DistributionListId, EmailContact};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct EventId(Uuid);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainEvent {
    pub event_id: EventId,
    pub version: u64,
    pub time: DateTime<Utc>,
    #[serde(flatten)]
    pub event: DomainEventType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum DomainEventType {
    ContactEvent(ContactEvent),
    DistributionListEvent(DistributionListEvent),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ContactEvent {
    pub contact_id: ContactId,
    pub event: ContactEventType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum ContactEventType {
    PersonAdded {
        first_name: Option<String>,
        last_name: Option<String>,
        email: Vec<EmailContact>,
    },
    OrganizationAdded {
        name: String,
        email: Vec<EmailContact>,
    },
    ContactRemoved,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DistributionListEvent {
    pub distribution_list_id: DistributionListId,
    pub event: DistributionListEventType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum DistributionListEventType {
    DistributionListAdded {
        name: String,
    },
    DistributionListRemoved,
    ContactAdded {
        contact_id: ContactId,
    },
    ContactRemoved {
        contact_id: ContactId,
    }
}
