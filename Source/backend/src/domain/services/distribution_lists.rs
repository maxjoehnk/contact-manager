use crate::db::DbContext;
use crate::db::repositories::*;
use crate::domain::*;

pub struct DistributionListsService;

impl DistributionListsService {
    pub async fn create(&self, db: &DbContext, distribution_list: CreateDistributionList) -> color_eyre::Result<DistributionListId> {
        let distribution_list_id = DistributionListId::new();
        let event = DistributionListEvent {
            distribution_list_id,
            event: DistributionListEventType::DistributionListAdded {
                name: distribution_list.name.clone(),
                contacts: distribution_list.contacts.clone(),
            },
        };
        let event = DomainEventType::DistributionListEvent(event);
        let event = DomainEvent::new(event, 1);

        EventsRepository.add_event(db, event).await?;
        DistributionListsRepository.create_distribution_list(db, DistributionList {
            id: distribution_list_id,
            name: distribution_list.name,
            contacts: distribution_list.contacts,
        }).await?;

        Ok(distribution_list_id)
    }

    pub async fn list(&self, db: &DbContext) -> color_eyre::Result<Vec<DistributionList>> {
        let distribution_lists = DistributionListsRepository.list_distribution_lists(db).await?;

        Ok(distribution_lists)
    }
}
