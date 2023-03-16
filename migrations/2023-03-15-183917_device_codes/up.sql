-- Your SQL goes here
CREATE TABLE device_codes (
  id SERIAL PRIMARY KEY,
  client_id UUID NOT NULL,
  user_code VARCHAR(8) NOT NULL,
  device_code VARCHAR(44) UNIQUE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  scopes TEXT[],
  CONSTRAINT device_code_client_id_fkey
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
    ON DELETE CASCADE
);

