-- Your SQL goes here
CREATE TABLE IF NOT EXISTS clients (
	id VARCHAR(32) PRIMARY KEY,
	secret TEXT,
	redirect_uri TEXT NOT NULL,
	is_public BOOLEAN NOT NULL,
	name TEXT NOT NULL,
	CONSTRAINT client_secret_check CHECK (
		(is_public AND secret IS NULL) OR
		(NOT is_public AND secret IS NOT NULL)
	)
);

