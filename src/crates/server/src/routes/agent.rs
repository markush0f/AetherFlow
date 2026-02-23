use crate::handlers::agent;
use axum::routing::get;
use sea_orm::DatabaseConnection;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<DatabaseConnection> {
    OpenApiRouter::new()
        .routes(routes!(agent::create_agent))
        .routes(routes!(agent::list_agents))
        .route("/{id}", get(agent::get_agent))
}
