use crate::models::agent;
use crate::repositories::agent::Repository as AgentRepository;
use aether_core::{Director, Runtime};
use sea_orm::*;
use uuid::Uuid;

pub struct Service;

impl Service {
    pub async fn create_agent(
        db: &DatabaseConnection,
        slug: String,
        endpoint: String,
    ) -> Result<agent::Model, DbErr> {
        let new_agent = agent::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            slug: Set(slug),
            endpoint: Set(endpoint),
            status: Set(agent::AgentStatus::Pending),
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

    /* Fetches the agent from the DB and sends the execution command to the Core.
       Translates the stored string runtime into the Core's Runtime enum.
    */
    pub async fn execute_agent_task(
        db: &DatabaseConnection,
        director: &Director,
        agent_id: String,
        payload: String,
    ) -> Result<String, String> {
        let agent = Self::get_agent_by_id(db, agent_id.clone())
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Agent {} not found", agent_id))?;

        /* We use the RemoteApi runtime for external webhooks */
        let runtime = Runtime::RemoteApi {
            endpoint: agent.endpoint.clone(),
            method: "POST".to_string(), // HTTP POST by default
        };

        let entrypoint = String::new(); // Not used for RemoteApi
        let workdir = String::new(); // Not used for RemoteApi

        /* Pass the dynamic data directly to the Core Actor System */
        director
            .execute_task(agent.id, runtime, entrypoint, workdir, payload)
            .await
    }
}
