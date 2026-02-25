-- Add endpoint and drop the old physical ones
ALTER TABLE agents ADD COLUMN IF NOT EXISTS endpoint TEXT NOT NULL DEFAULT 'http://localhost';
ALTER TABLE agents DROP COLUMN IF EXISTS command;
ALTER TABLE agents DROP COLUMN IF EXISTS runtime;
ALTER TABLE agents DROP COLUMN IF EXISTS workdir;
