use serde::{Deserialize, Serialize};

// The core Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    // Unique identifier for the agent
    pub id: String,
    // Human-readable slug for the API (e.g., "summarizer")
    pub slug: String,
    // The actual system command to run the agent
    pub command: String,
    // Current operational state
    pub status: AgentStatus,
}

// Enumeration of possible agent states
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AgentStatus {
    Idle,
    Running,
    Error,
    Offline,
}

// Data Transfer Objects (DTOs)

#[derive(Debug, Deserialize)]
pub struct AgentRequest {
    pub agent_id: String,
    pub input: String,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub output: String,
    pub error: Option<String>,
    pub status: String,
}
