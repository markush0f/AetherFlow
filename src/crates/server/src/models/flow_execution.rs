use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flow_executions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub flow_id: String,

    pub status: String,

    pub input_data: Option<serde_json::Value>,

    pub output_data: Option<serde_json::Value>,

    pub started_at: Option<DateTimeWithTimeZone>,

    pub completed_at: Option<DateTimeWithTimeZone>,
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
}

impl Related<crate::models::flow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flow.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
