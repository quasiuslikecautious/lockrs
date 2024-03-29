-- Your SQL goes here
CREATE TABLE device_authorizations (
  id SERIAL PRIMARY KEY,
  client_id VARCHAR(32) NOT NULL,
  user_code VARCHAR(8) UNIQUE NOT NULL,
  device_code VARCHAR(44) UNIQUE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  scopes TEXT[] NOT NULL,
  CONSTRAINT device_authorizations_client_id_fkey
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
    ON DELETE CASCADE,
  CONSTRAINT device_authorizations_scope_present CHECK (
    CARDINALITY(scopes) > 0
  )
);

CREATE INDEX device_authorizations_device_code_idx ON device_authorizations (device_code);

