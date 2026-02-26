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
    #[schema(value_type = Object)]
    pub payload: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
pub struct ExecuteAgentResponse {
    /// Agent's stdout response
    #[schema(value_type = Object)]
    pub response: serde_json::Value,
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
    // 1. Fetch the requested Agent from PostgreSQL database
    let agent_result = AgentService::get_agent_by_id(&state.db, id.clone()).await;

    match agent_result {
        Ok(Some(agent)) => {
            // 2. Perform resilient execution call to target agent node
            let result =
                AgentClient::execute_task(&state.http_client, &agent.endpoint, &payload.payload)
                    .await;

            // Extract valid JSON from the final response parsing
            let (response_json, retries_used) = match &result {
                Ok((res, retries)) => (res.clone(), *retries),
                Err((err_msg, retries)) => (serde_json::json!({ "error": err_msg }), *retries),
            };

            // 3. Log the task asynchronously in PostgreSQL so it never blocks the HTTP response to the client
            // We clone the necessary variables (DB pool, keys, values) to let Tokio spawn it separately
            let ping_db = state.db.clone();
            let agent_id_log = agent.id.clone();
            let payload_log = payload.payload.clone();
            tokio::spawn(async move {
                let _ = crate::services::agent_log::Service::log_task(
                    &ping_db,
                    agent_id_log,
                    payload_log,
                    response_json,
                    retries_used,
                )
                .await;
            });

            // 4. Return the outcome gracefully resolving the current request
            match result {
                Ok((response, _)) => {
                    (StatusCode::OK, Json(ExecuteAgentResponse { response })).into_response()
                }
                Err((e, _)) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
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
