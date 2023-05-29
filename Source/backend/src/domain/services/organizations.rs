use crate::db::DbContext;
use crate::db::repositories::*;
use crate::domain::*;

pub struct OrganizationsService;

impl OrganizationsService {
    pub async fn create(&self, db: &DbContext, organization: CreateOrganization) -> color_eyre::Result<ContactId> {
        let contact_id = ContactId::new();
        let event = ContactEvent {
            contact_id,
            event: ContactEventType::OrganizationAdded {
                name: organization.name.clone(),
                email: organization.email.clone(),
            },
        };
        let event = DomainEventType::ContactEvent(event);
        let event = DomainEvent::new(event, 1);

        EventsRepository.add_event(db, event).await?;
        ContactRepository.create_organization(db, Organization {
            id: contact_id,
            name: organization.name,
            email: organization.email,
        }).await?;

        Ok(contact_id)
    }

    pub async fn list(&self, db: &DbContext) -> color_eyre::Result<Vec<Organization>> {
        let organizations = ContactRepository.list_organizations(&db).await?;

        Ok(organizations)
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
        ContactRepository.delete_organization(db, id).await?;

        Ok(())
    }
}
