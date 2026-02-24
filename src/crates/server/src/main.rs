use dotenvy::dotenv;
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

// Base OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    components(
        schemas(models::agent::Model, models::agent::AgentStatus, handlers::agent::CreateAgentPayload)
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
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect for migrations");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations: SUCCESSFUL");

    println!("AetherFlow: Connecting with SeaORM...");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");
    println!("SeaORM: SUCCESSFUL");

    // Load the Router and collect API docs from routes
    let (router, api) = routes::create_router();

    // Merge the base doc with the routes doc
    let mut openapi = ApiDoc::openapi();
    openapi.merge(api);

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("AetherFlow active at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
