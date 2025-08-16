-- Add up migration script here
ALTER TABLE users ADD COLUMN public_key TEXT;

-- Add down migration script here
 ALTER TABLE users DROP COLUMN public_key;