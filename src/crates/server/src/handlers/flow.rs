use crate::models::{flow::Model as FlowModel, flow_step::Model as FlowStepModel};
use crate::services::flow::Service as FlowService;
use crate::services::flow_executor::Service as FlowExecutorService;
use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
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

#[utoipa::path(
    post,
    path = "/{id}/execute",
    params(
        ("id" = String, Path, description = "Flow database id")
    ),
    request_body = ExecuteFlowPayload,
    responses(
        (status = 200, description = "Flow executed successfully", body = ExecuteFlowResponse),
        (status = 500, description = "Internal server error")
    )
)]
/// Triggers the full cascaded execution of an Agent Flow, taking an initial JSON input.
pub async fn execute_flow_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ExecuteFlowPayload>,
) -> impl IntoResponse {
    let result = FlowExecutorService::execute_flow(
        &state.db,
        &state.http_client,
        id.clone(),
        payload.payload,
    )
    .await;

    match result {
        Ok(response) => (StatusCode::OK, Json(ExecuteFlowResponse { response })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/",
    request_body = CreateFlowPayload,
    responses(
        (status = 201, description = "Flow created successfully", body = FlowModel),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_flow(
    State(state): State<AppState>,
    Json(payload): Json<CreateFlowPayload>,
) -> impl IntoResponse {
    match FlowService::create_flow(&state.db, payload.name, payload.description).await {
        Ok(flow) => (StatusCode::CREATED, Json(flow)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "List all flows", body = [FlowModel]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_flows(State(state): State<AppState>) -> impl IntoResponse {
    match FlowService::get_all_flows(&state.db).await {
        Ok(flows) => (StatusCode::OK, Json(flows)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Flow database id")
    ),
    responses(
        (status = 200, description = "Flow found", body = FlowModel),
        (status = 404, description = "Flow not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_flow(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match FlowService::get_flow_by_id(&state.db, id).await {
        Ok(Some(flow)) => (StatusCode::OK, Json(flow)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Flow not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/{id}/steps",
    params(
        ("id" = String, Path, description = "Flow database id")
    ),
    request_body = CreateFlowStepPayload,
    responses(
        (status = 201, description = "Flow step created successfully", body = FlowStepModel),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_flow_step(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<CreateFlowStepPayload>,
) -> impl IntoResponse {
    match FlowService::add_flow_step(
        &state.db,
        id,
        payload.agent_id,
        payload.step_order,
        payload.config,
    )
    .await
    {
        Ok(step) => (StatusCode::CREATED, Json(step)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/{id}/steps",
    params(
        ("id" = String, Path, description = "Flow database id")
    ),
    responses(
        (status = 200, description = "List all steps for logic", body = [FlowStepModel]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_flow_steps(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match FlowService::get_flow_steps(&state.db, id).await {
        Ok(steps) => (StatusCode::OK, Json(steps)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
