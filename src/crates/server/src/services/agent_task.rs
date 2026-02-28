use crate::models::agent_task;
use crate::repositories::agent_task::Repository as AgentTaskRepository;

use sea_orm::*;
use uuid::Uuid;

pub struct Service;

impl Service {
    pub async fn create_task(
        db: &DatabaseConnection,
        agent_id: String,
        name: String,
        description: Option<String>,
        task_type: String,
        path: Option<String>,
        method: Option<String>,
        input_contract: Option<serde_json::Value>,
        output_contract: Option<serde_json::Value>,
        settings: Option<serde_json::Value>,
    ) -> Result<agent_task::Model, DbErr> {
        let new_task = agent_task::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            agent_id: Set(agent_id),
            name: Set(name),
            description: Set(description),
            task_type: Set(task_type),
            path: Set(path),
            method: Set(method),
            input_contract: Set(input_contract),
            output_contract: Set(output_contract),
            settings: Set(settings),
            created_at: Set(None),
        };

        AgentTaskRepository::create(db, new_task).await
    }

    pub async fn get_all_tasks(db: &DatabaseConnection) -> Result<Vec<agent_task::Model>, DbErr> {
        AgentTaskRepository::find_all(db).await
    }

    pub async fn get_tasks_for_agent(
        db: &DatabaseConnection,
        agent_id: String,
    ) -> Result<Vec<agent_task::Model>, DbErr> {
        AgentTaskRepository::find_by_agent_id(db, agent_id).await
    }

    pub async fn get_task_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<agent_task::Model>, DbErr> {
        AgentTaskRepository::find_by_id(db, id).await
    }

    pub async fn delete_task(db: &DatabaseConnection, id: String) -> Result<u64, DbErr> {
        AgentTaskRepository::delete(db, id).await
    }
}
