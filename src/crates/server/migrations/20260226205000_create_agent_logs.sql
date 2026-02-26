CREATE TABLE IF NOT EXISTS agent_logs (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    prompt JSONB NOT NULL,
    response JSONB NOT NULL,
    retries INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_agent
        FOREIGN KEY (agent_id) 
        REFERENCES agents(id)
        ON DELETE CASCADE
);
