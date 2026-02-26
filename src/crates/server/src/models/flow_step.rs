use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flow_steps")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub flow_id: String,

    pub agent_id: String,

    pub step_order: i32,

    pub config: Option<serde_json::Value>,

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
        belongs_to = "crate::models::agent::Entity",
        from = "Column::AgentId",
        to = "crate::models::agent::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Agent,
}

impl Related<crate::models::flow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flow.def()
    }
}

impl Related<crate::models::agent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
