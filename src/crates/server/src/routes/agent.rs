use crate::handlers::agent;
use axum::routing::{get, post};
use sea_orm::DatabaseConnection;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<DatabaseConnection> {
    // We define the routes here. OpenApiRouter will collect the metadata.
    OpenApiRouter::new()
        .route("/", post(agent::create_agent))
        .route("/", get(agent::list_agents))
        .route("/{id}", get(agent::get_agent))
}
