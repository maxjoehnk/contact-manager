use crate::db::DbContext;
use crate::db::repositories::*;
use crate::domain::*;

pub struct ContactsService;

impl ContactsService {
    pub async fn list(&self, db: &DbContext) -> color_eyre::Result<Vec<Contact>> {
        let contacts = ContactRepository.list_contacts(&db).await?;

        Ok(contacts)
    }
}
