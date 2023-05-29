use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::Contact;
use super::{ContactId, DistributionListId, EmailContact};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct EventId(Uuid);

impl From<Uuid> for EventId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<EventId> for Uuid {
    fn from(value: EventId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainEvent {
    pub event_id: EventId,
    pub version: u64,
    pub time: DateTime<Utc>,
    #[serde(flatten)]
    pub event: DomainEventType,
}

impl DomainEvent {
    pub fn new(event: DomainEventType, version: u64) -> Self {
        Self {
            event_id: EventId(Uuid::new_v4()),
            version,
            time: Utc::now(),
            event,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum DomainEventType {
    ContactEvent(ContactEvent),
    DistributionListEvent(DistributionListEvent),
}

impl DomainEventType {
    pub fn domain_id(&self) -> Uuid {
        match self {
            DomainEventType::ContactEvent(event) => event.contact_id.into(),
            DomainEventType::DistributionListEvent(event) => event.distribution_list_id.into(),
        }
    }
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
        contacts: Vec<Contact>,
    },
    DistributionListRemoved,
    ContactAdded {
        contact_id: ContactId,
    },
    ContactRemoved {
        contact_id: ContactId,
    }
}
