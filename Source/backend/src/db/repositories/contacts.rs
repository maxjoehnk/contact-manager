use sqlx::query;
use uuid::Uuid;

use crate::db::DbContext;
use crate::domain::{Contact, ContactId, Organization, Person};

pub struct ContactRepository;

impl ContactRepository {
    pub async fn create_person(&self, context: &DbContext, person: Person) -> color_eyre::Result<()> {
        query!(
            r#"INSERT INTO "Persons" (id, first_name, last_name) VALUES ($1, $2, $3)"#,
            Uuid::from(person.id),
            person.first_name,
            person.last_name,
        )
            .execute(context)
            .await?;

        Ok(())
    }

    pub async fn list_persons(&self, context: &DbContext) -> color_eyre::Result<Vec<Person>> {
        let persons = query!(
            r#"SELECT id, first_name, last_name FROM "Persons""#
        )
            .fetch_all(context)
            .await?;

        Ok(persons.into_iter().map(|person| Person {
            id: person.id.into(),
            first_name: person.first_name,
            last_name: person.last_name,
            email: Default::default(),
        }).collect())
    }

    pub async fn delete_person(&self, context: &DbContext, id: ContactId) -> color_eyre::Result<()> {
        query!(
            r#"DELETE FROM "Persons" WHERE id = $1"#,
            Uuid::from(id),
        ).execute(context).await?;

        Ok(())
    }

    pub async fn create_organization(&self, context: &DbContext, organization: Organization) -> color_eyre::Result<()> {
        query!(
            r#"INSERT INTO "Organizations" (id, name) VALUES ($1, $2)"#,
            Uuid::from(organization.id),
            organization.name,
        )
            .execute(context)
            .await?;

        Ok(())
    }

    pub async fn list_organizations(&self, context: &DbContext) -> color_eyre::Result<Vec<Organization>> {
        let organizations = query!(
            r#"SELECT id, name FROM "Organizations""#
        )
            .fetch_all(context)
            .await?;

        Ok(organizations.into_iter().map(|organization| Organization {
            id: organization.id.into(),
            name: organization.name,
            email: Default::default(),
        }).collect())
    }

    pub async fn delete_organization(&self, context: &DbContext, id: ContactId) -> color_eyre::Result<()> {
        query!(
            r#"DELETE FROM "Organizations" WHERE id = $1"#,
            Uuid::from(id),
        ).execute(context).await?;

        Ok(())
    }

    pub async fn list_contacts(&self, context: &DbContext) -> color_eyre::Result<Vec<Contact>> {
        let contacts = query!(
            r#"SELECT id, type, first_name, last_name, name FROM "Contacts""#
        )
            .fetch_all(context)
            .await?;

        Ok(contacts.into_iter().map(|contact| if contact.r#type.unwrap() == 1 {
            Contact::Person(Person {
                id: contact.id.unwrap().into(),
                first_name: contact.first_name,
                last_name: contact.last_name,
                email: Default::default(),
            })
        }else {
            Contact::Organization(Organization {
                id: contact.id.unwrap().into(),
                name: contact.name.unwrap(),
                email: Default::default(),
            })
        }).collect())
    }
}
