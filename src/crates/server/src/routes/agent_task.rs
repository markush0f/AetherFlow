use crate::handlers::agent_task;
use crate::state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(agent_task::create_task, agent_task::list_all_tasks))
        .routes(routes!(agent_task::list_tasks_for_agent))
        .routes(routes!(agent_task::get_task, agent_task::delete_task))
}
