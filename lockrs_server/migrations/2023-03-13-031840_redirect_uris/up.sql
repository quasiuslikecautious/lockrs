-- Your SQL goes here
CREATE TABLE IF NOT EXISTS redirect_uris (
	id SERIAL PRIMARY KEY,
	client_id VARCHAR(32) NOT NULL,
	uri TEXT NOT NULL,
	created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
	CONSTRAINT redirect_uris_client_id_fkey
		FOREIGN KEY (client_id)
		REFERENCES clients (id)
		ON DELETE CASCADE,
	CONSTRAINT redirect_uri_unique
		UNIQUE (client_id, uri)
);

CREATE INDEX redirect_uri_client_id_idx
	ON redirect_uris(client_id, uri);
