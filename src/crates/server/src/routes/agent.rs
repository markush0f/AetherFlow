use crate::handlers::{agent, gateway};
use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<AppState> {
    // We define the routes here. OpenApiRouter will collect the metadata.
    OpenApiRouter::new()
        .routes(routes!(agent::create_agent, agent::list_agents))
        .routes(routes!(agent::get_agent))
        .routes(routes!(gateway::execute_agent_task))
}
