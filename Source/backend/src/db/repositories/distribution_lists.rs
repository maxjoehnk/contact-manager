use itertools::Itertools;
use sqlx::query;
use uuid::Uuid;

use crate::db::DbContext;
use crate::domain::*;

pub struct DistributionListsRepository;

impl DistributionListsRepository {
    pub async fn create_distribution_list(&self, context: &DbContext, distribution_list: DistributionList) -> color_eyre::Result<()> {
        query!(
            r#"INSERT INTO "DistributionLists" (id, name) VALUES ($1, $2)"#,
            Uuid::from(distribution_list.id),
            distribution_list.name,
        )
            .execute(context)
            .await?;

        for (distribution_id, contact) in distribution_list.contacts.into_iter().map(|contact| (distribution_list.id, contact)) {
            match contact {
                Contact::Person(person) => {
                    query!(
                        r#"INSERT INTO "Person_DistributionList" (distribution_list_id, person_id) VALUES ($1, $2)"#,
                        Uuid::from(distribution_id),
                        Uuid::from(person.id),
                    )
                        .execute(context)
                        .await?;
                }
                Contact::Organization(organization) => {
                    query!(
                        r#"INSERT INTO "Organization_DistributionList" (distribution_list_id, organization_id) VALUES ($1, $2)"#,
                        Uuid::from(distribution_id),
                        Uuid::from(organization.id),
                    )
                        .execute(context)
                        .await?;
                }
            }
        }

        Ok(())
    }

    pub async fn list_distribution_lists(&self, context: &DbContext) -> color_eyre::Result<Vec<DistributionList>> {
        let records = query!(r#"
SELECT dl.id as "dl_id", dl.name as "dl_name", P.id as "p_id", P.first_name as "p_first_name", P.last_name as "p_last_name", O.id as "o_id", O.name as "o_name"
FROM "DistributionLists" as dl
         LEFT JOIN "Person_DistributionList" PDL on dl.id = PDL.distribution_list_id
         LEFT JOIN "Persons" P on P.id = PDL.person_id
         LEFT JOIN "Organization_DistributionList" ODL on dl.id = ODL.distribution_list_id
         LEFT JOIN "Organizations" O on O.id = ODL.organization_id"#)
            .fetch_all(context)
            .await?;

        let mut distribution_lists = Vec::new();
        for (id, records) in &records
            .into_iter()
            .group_by(|record| record.dl_id.unwrap()) {
            let records = records.collect::<Vec<_>>();
            distribution_lists.push(DistributionList {
                id: id.into(),
                name: records[0].dl_name.clone().unwrap(),
                contacts: records
                    .into_iter()
                    .filter(|record| record.p_id.is_some() || record.o_id.is_some())
                    .map(|record| {
                        if record.p_id.is_some() {
                            Contact::Person(Person {
                                id: record.p_id.unwrap().into(),
                                first_name: record.p_first_name,
                                last_name: record.p_last_name,
                                email: Default::default(),
                            })
                        } else {
                            Contact::Organization(Organization {
                                id: record.o_id.unwrap().into(),
                                name: record.o_name.unwrap(),
                                email: Default::default(),
                            })
                        }
                    })
                    .collect()
            });
        }

        Ok(distribution_lists)
    }
}
