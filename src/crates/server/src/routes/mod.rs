use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

mod agent_routes;

pub fn create_router() -> Router<DatabaseConnection> {
    Router::new()
        .route(
            "/health",
            get(|| async { "AetherFlow: Online (ORM Active)" }),
        )
        .nest("/agents", agent_routes::router())
}
