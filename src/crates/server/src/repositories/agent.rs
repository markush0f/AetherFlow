use crate::models::agent::{self, Entity as Agent};
use sea_orm::*;

pub struct Repository;

impl Repository {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<agent::Model>, DbErr> {
        Agent::find().all(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<agent::Model>, DbErr> {
        Agent::find_by_id(id).one(db).await
    }

    pub async fn create(
        db: &DatabaseConnection,
        data: agent::ActiveModel,
    ) -> Result<agent::Model, DbErr> {
        data.insert(db).await
    }

    pub async fn update_status(
        db: &DatabaseConnection,
        id: String,
        status: agent::AgentStatus,
    ) -> Result<Option<agent::Model>, DbErr> {
        let agent = Agent::find_by_id(id).one(db).await?;
        if let Some(agent) = agent {
            let mut active_agent: agent::ActiveModel = agent.into();
            active_agent.status = Set(status);
            let updated = active_agent.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
}
