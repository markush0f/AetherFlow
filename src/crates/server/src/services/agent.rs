use crate::models::agent;
use crate::repositories::agent::Repository as AgentRepository;

use sea_orm::*;
use uuid::Uuid;

pub struct Service;

impl Service {
    pub async fn create_agent(
        db: &DatabaseConnection,
        slug: String,
        endpoint: String,
        source: Option<String>,
    ) -> Result<agent::Model, DbErr> {
        let new_agent = agent::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            slug: Set(slug),
            endpoint: Set(endpoint),
            status: Set(agent::AgentStatus::Pending),
            source: Set(source),
        };

        AgentRepository::create(db, new_agent).await
    }

    pub async fn get_all_agents(db: &DatabaseConnection) -> Result<Vec<agent::Model>, DbErr> {
        AgentRepository::find_all(db).await
    }

    pub async fn get_agent_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<agent::Model>, DbErr> {
        AgentRepository::find_by_id(db, id).await
    }

    pub async fn update_status(
        db: &DatabaseConnection,
        id: String,
        status: agent::AgentStatus,
    ) -> Result<Option<agent::Model>, DbErr> {
        AgentRepository::update_status(db, id, status).await
    }
}
