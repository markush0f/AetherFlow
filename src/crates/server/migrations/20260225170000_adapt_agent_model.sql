-- Add endpoint and drop the old physical ones
ALTER TABLE agents ADD COLUMN endpoint TEXT NOT NULL DEFAULT 'http://localhost';
ALTER TABLE agents DROP COLUMN command;
ALTER TABLE agents DROP COLUMN runtime;
ALTER TABLE agents DROP COLUMN workdir;
