use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
#[serde(transparent)]
pub struct ContactId(Uuid);

impl ContactId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for ContactId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ContactId> for Uuid {
    fn from(value: ContactId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Contact {
    Person(Person),
    Organization(Organization),
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: ContactId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Vec<EmailContact>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub id: ContactId,
    pub name: String,
    pub email: Vec<EmailContact>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmailContact {
    pub label: String,
    pub email: Email,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[repr(transparent)]
pub struct Email(String);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePerson {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(default)]
    pub email: Vec<EmailContact>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganization {
    pub name: String,
    #[serde(default)]
    pub email: Vec<EmailContact>,
}
