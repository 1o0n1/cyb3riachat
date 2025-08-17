-- Add migration script here
ALTER TABLE users ADD COLUMN encrypted_private_key TEXT;