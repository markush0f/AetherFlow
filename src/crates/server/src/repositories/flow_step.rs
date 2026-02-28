use crate::models::flow_step::{self, Column, Entity as FlowStep};
use sea_orm::{QueryOrder, *};

pub struct Repository;

impl Repository {
    pub async fn create(
        db: &DatabaseConnection,
        flow_id: String,
        task_id: String,
        step_order: i32,
        config: Option<serde_json::Value>,
    ) -> Result<flow_step::Model, DbErr> {
        let step = flow_step::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            flow_id: Set(flow_id),
            task_id: Set(task_id),
            step_order: Set(step_order),
            config: Set(config),
            created_at: Set(None),
        };
        step.insert(db).await
    }

    pub async fn get_steps_for_flow(
        db: &DatabaseConnection,
        flow_id: String,
    ) -> Result<Vec<flow_step::Model>, DbErr> {
        FlowStep::find()
            .filter(Column::FlowId.eq(flow_id))
            .order_by_asc(Column::StepOrder)
            .all(db)
            .await
    }
}
