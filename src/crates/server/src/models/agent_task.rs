use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "agent_tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub agent_id: String,

    pub name: String,

    #[schema(value_type = Option<String>)]
    pub description: Option<String>,

    pub task_type: String,

    #[schema(value_type = Option<String>)]
    pub path: Option<String>,

    #[schema(value_type = Option<String>)]
    pub method: Option<String>,

    pub input_contract: Option<serde_json::Value>,

    pub output_contract: Option<serde_json::Value>,

    pub settings: Option<serde_json::Value>,

    #[schema(value_type = Option<String>)]
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::agent::Entity",
        from = "Column::AgentId",
        to = "crate::models::agent::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Agent,
    #[sea_orm(has_many = "crate::models::flow_step::Entity")]
    FlowStep,
}

impl Related<crate::models::agent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agent.def()
    }
}

impl Related<crate::models::flow_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlowStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
