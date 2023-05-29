use crate::db::DbContext;
use crate::db::repositories::*;
use crate::domain::*;

pub struct PersonsService;

impl PersonsService {
    pub async fn create(&self, db: &DbContext, person: CreatePerson) -> color_eyre::Result<ContactId> {
        let contact_id = ContactId::new();
        let event = ContactEvent {
            contact_id,
            event: ContactEventType::PersonAdded {
                first_name: person.first_name.clone(),
                last_name: person.last_name.clone(),
                email: person.email.clone(),
            },
        };
        let event = DomainEventType::ContactEvent(event);
        let event = DomainEvent::new(event, 1);

        EventsRepository.add_event(db, event).await?;
        ContactRepository.create_person(db, Person {
            id: contact_id,
            first_name: person.first_name,
            last_name: person.last_name,
            email: person.email,
        }).await?;

        Ok(contact_id)
    }

    pub async fn list(&self, db: &DbContext) -> color_eyre::Result<Vec<Person>> {
        let persons = ContactRepository.list_persons(db).await?;

        Ok(persons)
    }

    pub async fn delete(&self, db: &DbContext, id: ContactId) -> color_eyre::Result<()> {
        let version = EventsRepository.highest_version(db, id.into()).await?;
        let event = ContactEvent {
            contact_id: id,
            event: ContactEventType::ContactRemoved,
        };
        let event = DomainEventType::ContactEvent(event);
        let event = DomainEvent::new(event, version.saturating_add(1));

        EventsRepository.add_event(db, event).await?;
        ContactRepository.delete_person(db, id).await?;

        Ok(())
    }
}
