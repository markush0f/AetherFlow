use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AgentStatus {
    #[sea_orm(string_value = "Terminated")]
    Terminated,
    #[sea_orm(string_value = "Spawning")]
    Spawning,
    #[sea_orm(string_value = "Ready")]
    Ready,
    #[sea_orm(string_value = "Busy")]
    Busy,
    #[sea_orm(string_value = "Idle")]
    Idle,
    #[sea_orm(string_value = "Zombie")]
    Zombie,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "agents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub slug: String,
    pub command: String,
    pub runtime: String,
    pub status: AgentStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
