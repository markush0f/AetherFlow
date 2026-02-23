use crate::models::agent;
use crate::repositories::agent_repository::AgentRepository;
use sea_orm::*;
use uuid::Uuid;

pub struct AgentService;

impl AgentService {
    pub async fn create_agent(
        db: &DatabaseConnection,
        slug: String,
        command: String,
    ) -> Result<agent::Model, DbErr> {

        let new_agent = agent::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            slug: Set(slug),
            command: Set(command),
            status: Set("Idle".to_string()),
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
}
