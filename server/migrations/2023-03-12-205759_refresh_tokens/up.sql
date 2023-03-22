-- Your SQL goes here
CREATE TABLE IF NOT EXISTS refresh_tokens (
	id SERIAL PRIMARY KEY,
	token VARCHAR(44) NOT NULL UNIQUE,
	client_id VARCHAR(32) NOT NULL,
	created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
	expires_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
	used BOOLEAN NOT NULL DEFAULT false,
	scopes TEXT[] NOT NULL,
	CONSTRAINT refresh_tokens_client_id_fkey
		FOREIGN KEY (client_id)
		REFERENCES clients (id)
		ON DELETE CASCADE
);

CREATE INDEX refresh_tokens_token_idx ON refresh_tokens (token);

