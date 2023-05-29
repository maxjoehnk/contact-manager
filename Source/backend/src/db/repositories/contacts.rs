use std::ops::DerefMut;
use sqlx::query;
use crate::db::{DbConnnection, DbContext};
use crate::domain::Person;
use uuid::Uuid;

pub struct ContactRepository;

impl ContactRepository {
    pub async fn create_person(&self, transaction: &mut DbContext, person: Person) -> color_eyre::Result<()> {
        query!(
            r#"INSERT INTO "Persons" (id, first_name, last_name) VALUES ($1, $2, $3)"#,
            Uuid::from(person.id),
            person.first_name,
            person.last_name,
        )
            .execute(transaction.deref_mut())
            .await?;

        Ok(())
    }

    pub async fn list_persons(&self, context: &mut DbContext) -> color_eyre::Result<Vec<Person>> {
        todo!()
    }
}
