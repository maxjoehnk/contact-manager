use sqlx::types::Json;
use sqlx::query;
use crate::db::DbContext;
use crate::domain::*;
use uuid::Uuid;

pub struct EventsRepository;

impl EventsRepository {
    pub async fn add_event(&self, context: &DbContext, event: DomainEvent) -> color_eyre::Result<()> {
        query!(
            r#"INSERT INTO "Events" (event_id, domain_id, event, version) VALUES ($1, $2, $3, $4)"#,
            Uuid::from(event.event_id),
            event.event.domain_id(),
            Json(event.event) as _,
            event.version as i64,
        )
            .execute(context)
            .await?;

        Ok(())
    }

    pub async fn highest_version(&self, context: &DbContext, domain_id: Uuid) -> color_eyre::Result<u64> {
        let version = query!(
            r#"SELECT MAX(version) FROM "Events" WHERE domain_id = $1"#,
            domain_id,
        )
            .fetch_one(context)
            .await?
            .max
            .unwrap_or(0);

        Ok(version as u64)
    }
}
