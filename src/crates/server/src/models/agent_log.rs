use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "agent_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub agent_id: String,

    pub prompt: serde_json::Value,

    pub response: serde_json::Value,

    pub retries: i32,

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
}

impl Related<crate::models::agent::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
