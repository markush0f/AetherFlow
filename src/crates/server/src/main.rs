use axum::{routing::get, Router};
use std::net::SocketAddr;

mod models;

#[tokio::main]
async fn main() {
    // Basic route for status check
    let app = Router::new().route("/health", get(health_check));

    // Server address configuration
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("AetherFlow is starting on http://{}", addr);

    // Start the listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Launch the server
    axum::serve(listener, app).await.unwrap();
}

/* Handler to return a simple confirmation message */
async fn health_check() -> &'static str {
    "AetherFlow: Online"
}
