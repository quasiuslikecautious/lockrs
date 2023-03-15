-- Your SQL goes here
CREATE TABLE IF NOT EXISTS access_tokens (
	id SERIAL PRIMARY KEY,
	token VARCHAR(44) NOT NULL,
	client_id UUID NOT NULL,
	user_id UUID,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
	expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
	scopes TEXT[] NOT NULL,
	CONSTRAINT access_tokens_user_id_fkey
		FOREIGN KEY (user_id)
		REFERENCES users (id)
		ON DELETE CASCADE,
	CONSTRAINT access_tokens_client_id_fkey
		FOREIGN KEY (client_id)
		REFERENCES clients (id)
		ON DELETE CASCADE
);

CREATE INDEX access_tokens_token_idx ON access_tokens (token);

