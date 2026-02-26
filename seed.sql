-- AetherFlow Full Database Setup & Seed
-- To execute this via psql:
-- psql $DATABASE_URL -f seed.sql

-- 1. Ensure extensions for UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 2. Create the unified agents table
CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL,
    endpoint TEXT NOT NULL DEFAULT 'http://localhost',
    status TEXT NOT NULL,
    source TEXT
);

-- 3. Seed the default Agent Farm
INSERT INTO agents (id, slug, status, endpoint, source) VALUES
(gen_random_uuid()::text, 'translator', 'Pending', 'http://127.0.0.1:4000/api/translate', './agents/translator'),
(gen_random_uuid()::text, 'summarizer', 'Pending', 'http://127.0.0.1:4001/api/summarize', './agents/translator'),
(gen_random_uuid()::text, 'sentiment-analyzer', 'Pending', 'http://127.0.0.1:4002/api/sentiment', './agents/translator');
