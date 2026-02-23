use crate::handlers::agent_handler;
use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;

pub fn router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/", post(agent_handler::create_agent))
        .route("/", get(agent_handler::list_agents))
        .route("/:id", get(agent_handler::get_agent))
}
