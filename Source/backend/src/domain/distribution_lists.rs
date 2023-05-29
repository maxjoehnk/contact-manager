use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::contacts::Contact;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DistributionListId(Uuid);

impl DistributionListId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for DistributionListId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<DistributionListId> for Uuid {
    fn from(value: DistributionListId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DistributionList {
    pub id: DistributionListId,
    pub name: String,
    pub contacts: Vec<Contact>,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct CreateDistributionList {
    pub name: String,
    #[serde(default)]
    pub contacts: Vec<Contact>,
}
