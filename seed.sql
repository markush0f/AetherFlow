-- AetherFlow Database Seed: Default Farm Agents
-- To execute this seed via psql:
-- psql $DATABASE_URL -f seed.sql

INSERT INTO agents (id, slug, status, endpoint) VALUES
(gen_random_uuid()::text, 'translator', 'Pending', 'http://127.0.0.1:4000/api/translate'),
(gen_random_uuid()::text, 'summarizer', 'Pending', 'http://127.0.0.1:4001/api/summarize'),
(gen_random_uuid()::text, 'sentiment-analyzer', 'Pending', 'http://127.0.0.1:4002/api/sentiment');
