use crate::models::agent_task::Model as AgentTask;
use crate::services::agent_task::Service as AgentTaskService;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateAgentTaskPayload {
    pub agent_id: String,
    pub name: String,
    pub description: Option<String>,
    /// Task type: "endpoint", "function", "script", etc.
    pub task_type: String,
    /// HTTP path for endpoint tasks (e.g. "/api/generate")
    pub path: Option<String>,
    /// HTTP method for endpoint tasks (e.g. "POST")
    pub method: Option<String>,
    /// JSON Schema describing expected input
    #[schema(value_type = Option<Object>)]
    pub input_contract: Option<serde_json::Value>,
    /// JSON Schema describing expected output
    #[schema(value_type = Option<Object>)]
    pub output_contract: Option<serde_json::Value>,
    /// Arbitrary JSON settings for the task
    #[schema(value_type = Option<Object>)]
    pub settings: Option<serde_json::Value>,
}

#[utoipa::path(
    post,
    path = "/",
    request_body = CreateAgentTaskPayload,
    responses(
        (status = 201, description = "Task created successfully", body = AgentTask),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateAgentTaskPayload>,
) -> impl IntoResponse {
    match AgentTaskService::create_task(
        &state.db,
        payload.agent_id,
        payload.name,
        payload.description,
        payload.task_type,
        payload.path,
        payload.method,
        payload.input_contract,
        payload.output_contract,
        payload.settings,
    )
    .await
    {
        Ok(task) => (StatusCode::CREATED, Json(task)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List all tasks", body = [AgentTask]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_all_tasks(State(state): State<AppState>) -> impl IntoResponse {
    match AgentTaskService::get_all_tasks(&state.db).await {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/agent/{agent_id}",
    params(
        ("agent_id" = String, Path, description = "Agent database id")
    ),
    responses(
        (status = 200, description = "List tasks for agent", body = [AgentTask]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_tasks_for_agent(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
) -> impl IntoResponse {
    match AgentTaskService::get_tasks_for_agent(&state.db, agent_id).await {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Task database id")
    ),
    responses(
        (status = 200, description = "Task found", body = AgentTask),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_task(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match AgentTaskService::get_task_by_id(&state.db, id).await {
        Ok(Some(task)) => (StatusCode::OK, Json(task)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Task not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Task database id")
    ),
    responses(
        (status = 200, description = "Task deleted"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match AgentTaskService::delete_task(&state.db, id).await {
        Ok(rows) if rows > 0 => (StatusCode::OK, "Task deleted").into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, "Task not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
