use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "flow_steps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub flow_id: String,

    pub task_id: String,

    pub step_order: i32,

    pub config: Option<serde_json::Value>,

    #[schema(value_type = Option<String>)]
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::flow::Entity",
        from = "Column::FlowId",
        to = "crate::models::flow::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Flow,
    #[sea_orm(
        belongs_to = "crate::models::agent_task::Entity",
        from = "Column::TaskId",
        to = "crate::models::agent_task::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    AgentTask,
}

impl Related<crate::models::flow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flow.def()
    }
}

impl Related<crate::models::agent_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AgentTask.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize, ToSchema)]
pub struct CreateFlowStepPayload {
    pub task_id: String,
    pub step_order: i32,
    pub config: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct FlowStepWithTask {
    #[serde(flatten)]
    pub step: Model,
    pub task_name: String,
}
