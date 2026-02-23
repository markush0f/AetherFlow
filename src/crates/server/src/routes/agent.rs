use crate::handlers::agent_handler;
use axum::routing::get;
use sea_orm::DatabaseConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<DatabaseConnection> {
    // OpenApiRouter automatically collects the metadata from the handlers
    OpenApiRouter::new()
        .routes(routes!(agent_handler::create_agent))
        .routes(routes!(agent_handler::list_agents))
        .route("/{id}", get(agent_handler::get_agent)) // This one needs manual metadata if not using utoipa_axum::routes
}
