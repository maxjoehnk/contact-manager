use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ContactId(Uuid);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct DistributionListId(Uuid);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Contact {
    Person(Person),
    Organization(Organization)
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Person {
    pub id: ContactId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Vec<EmailContact>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct Organization {
    pub id: ContactId,
    pub name: String,
    pub email: Vec<EmailContact>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct EmailContact {
    pub label: String,
    pub email: Email,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
pub struct Email(String);

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct DistributionList {
    pub id: Uuid,
    pub name: String,
    pub contacts: Vec<Contact>,
}
