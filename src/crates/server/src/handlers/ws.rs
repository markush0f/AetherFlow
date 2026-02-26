use crate::services::agent::Service as AgentService;
use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use std::time::Duration;
use tokio::time::interval;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    // Upgrades the HTTP request to a WebSocket connection
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    // Interval ticker to push updates to the client every 2 seconds
    let mut ticker = interval(Duration::from_secs(2));

    loop {
        ticker.tick().await;

        match AgentService::get_all_agents(&state.db).await {
            Ok(agents) => {
                // Serialize agents list and send it securely through the WebSocket tunnel
                if let Ok(json) = serde_json::to_string(&agents) {
                    if socket.send(Message::Text(json.into())).await.is_err() {
                        // Connection was closed by the client
                        break;
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}
