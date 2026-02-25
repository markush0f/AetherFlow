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
        command: String,
        runtime: String,
        workdir: String,
    ) -> Result<agent::Model, DbErr> {
        let new_agent = agent::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            slug: Set(slug),
            command: Set(command),
            runtime: Set(runtime),
            workdir: Set(workdir),
            status: Set(agent::AgentStatus::Idle),
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

        /* Map the string from the database to our Core's Runtime enum dynamically */
        let runtime = match agent.runtime.as_str() {
            "python3" => Runtime::Python3,
            "node" => Runtime::NodeJS,
            "binary" => Runtime::Native,
            _ => return Err(format!("Unsupported runtime in DB: {}", agent.runtime)),
        };

        let entrypoint = agent.command;
        let workdir = agent.workdir;

        /* Pass the dynamic data directly to the Core Actor System */
        director
            .execute_task(agent.id, runtime, entrypoint, workdir, payload)
            .await
    }
}
