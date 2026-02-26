use aether_core::Director;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub director: Director,
    pub http_client: reqwest::Client,
}
