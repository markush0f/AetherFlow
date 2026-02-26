use dotenvy::dotenv;
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
pub mod state;

// Base OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    components(
        schemas(models::agent::Model, models::agent::AgentStatus, handlers::agent::CreateAgentPayload, handlers::agent::ExecuteAgentPayload, handlers::agent::ExecuteAgentResponse)
    ),
    tags(
        (name = "AetherFlow", description = "Agent Management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aether_server=info,tower_http=info,info".into()),
        )
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    info!("AetherFlow: Running migrations...");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect for migrations");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    info!("Migrations: SUCCESSFUL");

    info!("AetherFlow: Connecting with SeaORM...");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");
    info!("SeaORM: SUCCESSFUL");

    info!("AetherFlow: Starting Director Engine...");
    let director = aether_core::Director::new();

    let app_state = state::AppState { db, director };

    // Load the Router and collect API docs from routes
    let (router, api) = routes::create_router();

    // Merge the base doc with the routes doc
    let mut openapi = ApiDoc::openapi();
    openapi.merge(api);

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("AetherFlow active at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
