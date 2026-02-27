use crate::models::flow_execution::{self, Entity as FlowExecution};
use sea_orm::*;
use uuid::Uuid;

pub struct Repository;

impl Repository {
    pub async fn create(
        db: &DatabaseConnection,
        flow_id: String,
        input_data: Option<serde_json::Value>,
    ) -> Result<flow_execution::Model, DbErr> {
        let execution = flow_execution::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            flow_id: Set(flow_id),
            status: Set("Running".to_string()),
            input_data: Set(input_data),
            output_data: Set(None),
            started_at: Set(None), // DB handles default
            completed_at: Set(None),
        };
        execution.insert(db).await
    }

    pub async fn update_status(
        db: &DatabaseConnection,
        id: String,
        status: &str,
        output_data: Option<serde_json::Value>,
    ) -> Result<Option<flow_execution::Model>, DbErr> {
        let exec = FlowExecution::find_by_id(id.clone()).one(db).await?;
        if let Some(exec) = exec {
            let mut active_exec: flow_execution::ActiveModel = exec.into();
            active_exec.status = Set(status.to_string());
            if output_data.is_some() {
                active_exec.output_data = Set(output_data);
            }
            // If status is terminal, set completed_at
            if status == "Completed" || status == "Failed" {
                active_exec.completed_at = Set(Some(chrono::Utc::now().into()));
            }
            let updated = active_exec.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
}
