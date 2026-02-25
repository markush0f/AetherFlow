use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/*
 * Represents the network health and lifecycle state of the external AI agent.
 */
#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum AgentStatus {
    /* The agent is registered but the endpoint has not been verified yet */
    #[sea_orm(string_value = "Pending")]
    Pending,

    /* The agent endpoint responded successfully to the health check */
    #[sea_orm(string_value = "Ready")]
    Ready,

    /* The agent endpoint is down, timed out, or returning errors */
    #[sea_orm(string_value = "Unreachable")]
    Unreachable,
}

/*
 * It stores the routing information required to forward requests to the agent.
 */
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "agents")]
pub struct Model {
    /* Unique identifier for the agent record */
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /* The public identifier used in the URL routing (e.g., "sales-agent") */
    pub slug: String,

    /* The full HTTP webhook URL where the external agent receives tasks */
    pub endpoint: String,

    /* The current network status of the agent */
    pub status: AgentStatus,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
