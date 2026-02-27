use crate::handlers::flow;
use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(flow::list_flows, flow::create_flow))
        .routes(routes!(flow::get_flow))
        .routes(routes!(flow::get_flow_steps, flow::add_flow_step))
        .routes(routes!(flow::execute_flow_task))
}
