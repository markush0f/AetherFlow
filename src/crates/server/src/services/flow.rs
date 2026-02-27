use crate::models::{flow, flow_step};
use crate::repositories::{
    flow::Repository as FlowRepository, flow_step::Repository as FlowStepRepository,
};
use sea_orm::{DatabaseConnection, DbErr};

pub struct Service;

impl Service {
    pub async fn create_flow(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
    ) -> Result<flow::Model, DbErr> {
        FlowRepository::create(db, name, description).await
    }

    pub async fn get_all_flows(db: &DatabaseConnection) -> Result<Vec<flow::Model>, DbErr> {
        FlowRepository::find_all(db).await
    }

    pub async fn get_flow_by_id(
        db: &DatabaseConnection,
        id: String,
    ) -> Result<Option<flow::Model>, DbErr> {
        FlowRepository::find_by_id(db, id).await
    }

    pub async fn add_flow_step(
        db: &DatabaseConnection,
        flow_id: String,
        agent_id: String,
        step_order: i32,
        config: Option<serde_json::Value>,
    ) -> Result<flow_step::Model, DbErr> {
        FlowStepRepository::create(db, flow_id, agent_id, step_order, config).await
    }

    pub async fn get_flow_steps(
        db: &DatabaseConnection,
        flow_id: String,
    ) -> Result<Vec<flow_step::Model>, DbErr> {
        FlowStepRepository::get_steps_for_flow(db, flow_id).await
    }
}
