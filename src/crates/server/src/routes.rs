use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

mod agent;

pub fn create_router() -> (Router<DatabaseConnection>, OpenApi) {
    // We create the router and collect the OpenAPI documentation
    let (router, api) = OpenApiRouter::new()
        .route(
            "/health",
            get(|| async { "AetherFlow: Online (ORM Active)" }),
        )
        .nest("/agents", agent::router())
        .split_for_parts();

    (router, api)
}
