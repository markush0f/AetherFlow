use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;
use utoipa_axum::router::OpenApiRouter;

mod agent;

pub fn create_router() -> Router<DatabaseConnection> {
    // We convert the OpenApiRouter to a regular Axum Router
    let (router, _api) = OpenApiRouter::new()
        .route(
            "/health",
            get(|| async { "AetherFlow: Online (ORM Active)" }),
        )
        .nest("/agents", agent::router())
        .split_for_parts();

    router
}
