use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    // 1. Load the .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    // 2. Attempt to connect to the database
    println!("AetherFlow: Connecting to database...");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(p) => {
            println!("Database connection: SUCCESSFUL");
            p
        }
        Err(e) => {
            println!("Database connection: FAILED");
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // 3. Simple router to keep the server alive
    let app = Router::new()
        .route("/health", get(|| async { "DB Connected" }))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("AetherFlow active at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
