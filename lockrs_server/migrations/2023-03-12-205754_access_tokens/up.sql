-- Your SQL goes here
CREATE TABLE IF NOT EXISTS access_tokens (
	id SERIAL PRIMARY KEY,
	token VARCHAR(128) NOT NULL,
	client_id VARCHAR(32) NOT NULL,
  user_id UUID,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
	expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
	scopes TEXT[] NOT NULL,
	CONSTRAINT access_tokens_client_id_fkey
		FOREIGN KEY (client_id)
		REFERENCES clients (id)
		ON DELETE CASCADE,
  CONSTRAINT access_tokens_user_id_fkey
    FOREIGN KEY (user_id)
    REFERENCES users (id)
    ON DELETE CASCADE,
  CONSTRAINT access_tokens_min_token_length CHECK (
    LENGTH(token) >= 43
  ),
  CONSTRAINT access_tokens_scope_present CHECK (
    CARDINALITY(scopes) > 0
  )
);

CREATE INDEX access_tokens_token_idx ON access_tokens (token);

