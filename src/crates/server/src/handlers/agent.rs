use crate::models::agent::Model as Agent;
use crate::services::agent::Service as AgentService;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateAgentPayload {
    /// Slug identifier for the agent
    pub slug: String,
    /// The full HTTP webhook URL where the external agent receives tasks
    pub endpoint: String,
    /// Optional source location
    pub source: Option<String>,
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
// Creates a new Agent in the database, expecting a slug, endpoint and optional source.
pub async fn create_agent(
    State(state): State<AppState>,
    Json(payload): Json<CreateAgentPayload>,
) -> impl IntoResponse {
    match AgentService::create_agent(&state.db, payload.slug, payload.endpoint, payload.source)
        .await
    {
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
// Retrieves a full list of all available Agents from the SeaORM database.
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
// Fetches a specific Agent record directly by its unique UUID.
pub async fn get_agent(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match AgentService::get_agent_by_id(&state.db, id).await {
        Ok(Some(agent)) => (StatusCode::OK, Json(agent)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Agent not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
