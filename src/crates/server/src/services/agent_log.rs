use crate::models::agent_log;
use crate::repositories::agent_log::Repository as AgentLogRepository;

use sea_orm::*;
use uuid::Uuid;

pub struct Service;

impl Service {
    /// Inserts a new task log entry into the database.
    /// Used by the gateway to permanently trace all AI executions.
    pub async fn log_task(
        db: &DatabaseConnection,
        agent_id: String,
        prompt: serde_json::Value,
        response: serde_json::Value,
        retries: i32,
    ) -> Result<agent_log::Model, DbErr> {
        let log = agent_log::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            agent_id: Set(agent_id),
            prompt: Set(prompt),
            response: Set(response),
            retries: Set(retries),
            created_at: Set(None), // DB handles default timestamp
        };

        // Persist the log using SeaORM
        AgentLogRepository::create(db, log).await
    }
}
