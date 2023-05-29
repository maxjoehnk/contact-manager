use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::contacts::Contact;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DistributionListId(Uuid);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DistributionList {
    pub id: DistributionListId,
    pub name: String,
    pub contacts: Vec<Contact>,
}
