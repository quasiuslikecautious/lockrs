-- Your SQL goes here
CREATE TABLE authorization_codes (
  id SERIAL PRIMARY KEY,
  code VARCHAR(100) NOT NULL,
  client_id VARCHAR(32) NOT NULL,
  user_id UUID NOT NULL,
  redirect_uri TEXT NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  used BOOLEAN NOT NULL DEFAULT FALSE,
  CONSTRAINT authorization_codes_client_id_fkey
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
    ON DELETE CASCADE,
  CONSTRAINT authorization_codes_redirect_uri_fkey
    FOREIGN KEY (client_id, redirect_uri)
    REFERENCES redirect_uris (client_id, uri)
    ON DELETE CASCADE,
  CONSTRAINT authorization_codes_user_id
    FOREIGN KEY (user_id)
    REFERENCES users (id)
    ON DELETE CASCADE
)

