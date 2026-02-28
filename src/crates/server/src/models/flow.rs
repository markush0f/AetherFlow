use crate::models::flow_step::FlowStepWithTask;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "flows")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub name: String,

    pub description: Option<String>,

    #[schema(value_type = Option<String>)]
    pub created_at: Option<DateTimeWithTimeZone>,

    #[schema(value_type = Option<String>)]
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::models::flow_step::Entity")]
    FlowStep,
    #[sea_orm(has_many = "crate::models::flow_execution::Entity")]
    FlowExecution,
}

impl Related<crate::models::flow_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowStep.def()
    }
}

impl Related<crate::models::flow_execution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowExecution.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize, ToSchema)]
pub struct CreateFlowPayload {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ExecuteFlowPayload {
    /// Initial payload to send to the first agent in the flow sequence
    #[schema(value_type = Object)]
    pub payload: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
pub struct ExecuteFlowResponse {
    /// Final compounded sequence output
    #[schema(value_type = Object)]
    pub response: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
pub struct FlowWithSteps {
    #[serde(flatten)]
    pub flow: Model,
    pub steps: Vec<FlowStepWithTask>,
    pub agents_chain: Vec<String>,
}
