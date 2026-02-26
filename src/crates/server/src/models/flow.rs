use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flows")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    pub name: String,

    pub description: Option<String>,

    pub created_at: Option<DateTimeWithTimeZone>,

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
