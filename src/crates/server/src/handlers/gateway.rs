use crate::services::agent::Service as AgentService;
use crate::services::agent_client::Service as AgentClient;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ExecuteAgentPayload {
    /// Payload to send to the agent's target endpoint
    pub payload: String,
}

#[derive(Serialize, ToSchema)]
pub struct ExecuteAgentResponse {
    /// Agent's stdout response
    pub response: String,
}

#[utoipa::path(
    post,
    path = "/{id}/execute",
    params(
        ("id" = String, Path, description = "Agent database id")
    ),
    request_body = ExecuteAgentPayload,
    responses(
        (status = 200, description = "Task executed successfully", body = ExecuteAgentResponse),
        (status = 404, description = "Agent not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn execute_agent_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ExecuteAgentPayload>,
) -> impl IntoResponse {
    let agent_result = AgentService::get_agent_by_id(&state.db, id.clone()).await;

    match agent_result {
        Ok(Some(agent)) => {
            match AgentClient::execute_task(&state.http_client, &agent.endpoint, &payload.payload)
                .await
            {
                Ok(response) => {
                    (StatusCode::OK, Json(ExecuteAgentResponse { response })).into_response()
                }
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
            }
        }
        Ok(None) => (StatusCode::NOT_FOUND, format!("Agent {} not found", id)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}
