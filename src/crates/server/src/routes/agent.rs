use crate::handlers::agent;
use crate::state::AppState;
use axum::routing::{get, post};
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<AppState> {
    // We define the routes here. OpenApiRouter will collect the metadata.
    OpenApiRouter::new()
        .route("/", post(agent::create_agent))
        .route("/", get(agent::list_agents))
        .route("/{id}", get(agent::get_agent))
        .route("/{id}/execute", post(agent::execute_agent_task))
}
