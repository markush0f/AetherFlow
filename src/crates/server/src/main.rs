use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("AetherFlow: Running migrations...");
    // Seguimos usando sqlx para las migraciones por ahora ya que es muy directo
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect for migrations");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations: SUCCESSFUL");

    // Inicializamos SeaORM
    println!("AetherFlow: Connecting with SeaORM...");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");
    println!("SeaORM: SUCCESSFUL");

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "AetherFlow: Online (ORM Active)" }),
        )
        .route("/agents", post(handlers::agent_handler::create_agent))
        .route("/agents", get(handlers::agent_handler::list_agents))
        .route("/agents/:id", get(handlers::agent_handler::get_agent))
        .with_state(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("AetherFlow active at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
