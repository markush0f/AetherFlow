use dotenvy::dotenv;
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};
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
        schemas(
            models::agent::Model, models::agent::AgentStatus,
            models::flow::Model, models::flow_step::Model,
            handlers::agent::CreateAgentPayload,
            handlers::gateway::ExecuteAgentPayload, handlers::gateway::ExecuteAgentResponse,
            models::flow_payloads::ExecuteFlowPayload, models::flow_payloads::ExecuteFlowResponse,
            models::flow_payloads::CreateFlowPayload, models::flow_payloads::CreateFlowStepPayload,
            models::flow_payloads::FlowStepWithAgent, models::flow_payloads::FlowWithSteps
        )
    ),
    tags(
        (name = "AetherFlow", description = "Agent Management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Load environment variables from the `.env` file if present
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aether_server=info,tower_http=info,info".into()),
        )
        .init();

    // Fetch database connection string from environment
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

    // Set up an HTTP client with a 30s timeout to communicate with our agents
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    // Bundle context dependencies to inject into Axum handlers
    let app_state = state::AppState {
        db: db.clone(),
        director,
        http_client: http_client.clone(),
    };

    // Spawn the background worker that pings agents to monitor their health
    services::monitor::Monitor::start_health_check(db.clone(), http_client.clone());

    // Load the Router and collect API docs from routes
    let (router, api) = routes::create_router();

    // Merge the base doc with the routes doc
    let mut openapi = ApiDoc::openapi();
    openapi.merge(api);

    // Build the Axum application router
    let app = router
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
        // Apply CORS middleware to allow requests from our web dashboard
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("AetherFlow active at http://{}", addr);

    // Bind the TCP listener and start serving the Axum web application
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
