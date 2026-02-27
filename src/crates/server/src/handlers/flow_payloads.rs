use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateFlowPayload {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateFlowStepPayload {
    pub agent_id: String,
    pub step_order: i32,
    pub config: Option<serde_json::Value>,
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
