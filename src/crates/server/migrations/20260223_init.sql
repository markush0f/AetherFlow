CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL,
    command TEXT NOT NULL,
    status TEXT NOT NULL
);
