-- Table for defining a flow (pipeline)
CREATE TABLE IF NOT EXISTS flows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table for defining the sequence of agents in a flow
CREATE TABLE IF NOT EXISTS flow_steps (
    id TEXT PRIMARY KEY,
    flow_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    step_order INTEGER NOT NULL,
    config JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_flow
        FOREIGN KEY (flow_id) 
        REFERENCES flows(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_agent
        FOREIGN KEY (agent_id) 
        REFERENCES agents(id)
        ON DELETE CASCADE
);

-- Table for tracking the execution of a flow
CREATE TABLE IF NOT EXISTS flow_executions (
    id TEXT PRIMARY KEY,
    flow_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending', -- Pending, Running, Completed, Failed
    input_data JSONB,
    output_data JSONB,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_flow_execution
        FOREIGN KEY (flow_id)
        REFERENCES flows(id)
        ON DELETE CASCADE
);
