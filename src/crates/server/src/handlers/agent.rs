use crate::models::agent::Model as Agent;
use crate::services::agent::Service as AgentService;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateAgentPayload {
    /// Slug identifier for the agent
    pub slug: String,
    /// System command associated with the agent
    pub command: String,
}

#[utoipa::path(
    post,
    path = "/agents",
    request_body = CreateAgentPayload,
    responses(
        (status = 201, description = "Agent created successfully", body = Agent),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_agent(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateAgentPayload>,
) -> impl IntoResponse {
    match AgentService::create_agent(&db, payload.slug, payload.command).await {
        Ok(agent) => (StatusCode::CREATED, Json(agent)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/agents",
    responses(
        (status = 200, description = "List all agents", body = [Agent]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_agents(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    match AgentService::get_all_agents(&db).await {
        Ok(agents) => (StatusCode::OK, Json(agents)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/agents/{id}",
    params(
        ("id" = String, Path, description = "Agent database id")
    ),
    responses(
        (status = 200, description = "Agent found", body = Agent),
        (status = 404, description = "Agent not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_agent(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match AgentService::get_agent_by_id(&db, id).await {
        Ok(Some(agent)) => (StatusCode::OK, Json(agent)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Agent not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
