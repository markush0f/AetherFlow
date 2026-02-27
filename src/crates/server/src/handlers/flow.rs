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
