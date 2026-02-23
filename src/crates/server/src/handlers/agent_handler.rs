use crate::services::agent_service::AgentService;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAgentPayload {
    pub slug: String,
    pub command: String,
}

pub async fn create_agent(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateAgentPayload>,
) -> impl IntoResponse {
    match AgentService::create_agent(&db, payload.slug, payload.command).await {
        Ok(agent) => (StatusCode::CREATED, Json(agent)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn list_agents(State(db): State<DatabaseConnection>) -> impl IntoResponse {
    match AgentService::get_all_agents(&db).await {
        Ok(agents) => (StatusCode::OK, Json(agents)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

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
