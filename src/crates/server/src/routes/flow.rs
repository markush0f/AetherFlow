use crate::handlers::flow;
use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(flow::execute_flow_task))
}
