use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineState {
    pub status: String,
    pub uptime: u64,
}

pub fn get_initial_state() -> EngineState {
    EngineState {
        status: "initialized".to_string(),
        uptime: 0,
    }
}
