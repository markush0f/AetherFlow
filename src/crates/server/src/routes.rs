use crate::state::AppState;
use axum::routing::get;
use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

mod agent;

pub fn create_router() -> (Router<AppState>, OpenApi) {
    // We create the router and collect the OpenAPI documentation
    let (router, api) = OpenApiRouter::new()
        .route(
            "/health",
            get(|| async { "AetherFlow: Online (ORM Active)" }),
        )
        .route("/ws", get(crate::handlers::ws::ws_handler))
        .nest("/agents", agent::router())
        .split_for_parts();

    (router, api)
}
