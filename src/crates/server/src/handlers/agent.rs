use crate::models::agent::Model as Agent;
use crate::services::agent::Service as AgentService;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateAgentPayload {
    /// Slug identifier for the agent
    pub slug: String,
    /// The full HTTP webhook URL where the external agent receives tasks
    pub endpoint: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ExecuteAgentPayload {
    /// Payload to send to the agent's stdin
    pub payload: String,
}

#[derive(Serialize, ToSchema)]
pub struct ExecuteAgentResponse {
    /// Agent's stdout response
    pub response: String,
}

#[utoipa::path(
    post,
    path = "/",
    request_body = CreateAgentPayload,
    responses(
        (status = 201, description = "Agent created successfully", body = Agent),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_agent(
    State(state): State<AppState>,
    Json(payload): Json<CreateAgentPayload>,
) -> impl IntoResponse {
    match AgentService::create_agent(&state.db, payload.slug, payload.endpoint).await {
        Ok(agent) => (StatusCode::CREATED, Json(agent)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List all agents", body = [Agent]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_agents(State(state): State<AppState>) -> impl IntoResponse {
    match AgentService::get_all_agents(&state.db).await {
        Ok(agents) => (StatusCode::OK, Json(agents)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Agent database id")
    ),
    responses(
        (status = 200, description = "Agent found", body = Agent),
        (status = 404, description = "Agent not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_agent(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match AgentService::get_agent_by_id(&state.db, id).await {
        Ok(Some(agent)) => (StatusCode::OK, Json(agent)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Agent not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
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
    match AgentService::execute_agent_task(&state.db, &state.director, id, payload.payload).await {
        Ok(response) => {
            let res = ExecuteAgentResponse { response };
            (StatusCode::OK, Json(res)).into_response()
        }
        Err(e) => {
            if e.contains("not found") {
                (StatusCode::NOT_FOUND, e).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
            }
        }
    }
}
