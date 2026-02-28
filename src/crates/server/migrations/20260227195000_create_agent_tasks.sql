CREATE TABLE IF NOT EXISTS agent_tasks (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    task_type TEXT NOT NULL, 
    path TEXT, 
    method TEXT,
    input_contract JSONB,
    output_contract JSONB,
    settings JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_agent
        FOREIGN KEY (agent_id) 
        REFERENCES agents(id)
        ON DELETE CASCADE
);

TRUNCATE TABLE flow_steps CASCADE;

ALTER TABLE flow_steps DROP CONSTRAINT fk_agent;
ALTER TABLE flow_steps RENAME COLUMN agent_id TO task_id;
ALTER TABLE flow_steps ADD CONSTRAINT fk_task
    FOREIGN KEY (task_id)
    REFERENCES agent_tasks(id)
    ON DELETE CASCADE;
