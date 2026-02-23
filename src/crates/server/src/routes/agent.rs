use crate::handlers::agent_handler;
use axum::routing::get;
use sea_orm::DatabaseConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<DatabaseConnection> {
    OpenApiRouter::new()
        .routes(routes!(agent_handler::create_agent))
        .routes(routes!(agent_handler::list_agents))
        .route("/{id}", get(agent_handler::get_agent)) 
}
