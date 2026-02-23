use dotenvy::dotenv;
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod models;
mod routes;
mod services;

// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::agent_handler::create_agent,
        handlers::agent_handler::list_agents,
        handlers::agent_handler::get_agent,
    ),
    components(
        schemas(models::agent::Model, handlers::agent_handler::CreateAgentPayload)
    ),
    tags(
        (name = "AetherFlow", description = "Agent Management API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("AetherFlow: Running migrations...");
    // We continue using sqlx for migrations for now as it is very straightforward
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect for migrations");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations: SUCCESSFUL");

    // Initialize SeaORM
    println!("AetherFlow: Connecting with SeaORM...");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");
    println!("SeaORM: SUCCESSFUL");

    // Load the Router and merge Swagger UI
    let app = routes::create_router()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("AetherFlow active at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
