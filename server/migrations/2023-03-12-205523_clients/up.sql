-- Your SQL goes here
CREATE TABLE IF NOT EXISTS clients (
	id VARCHAR(32) PRIMARY KEY,
	secret TEXT,
  user_id UUID NOT NULL,
	is_public BOOLEAN NOT NULL,
	name TEXT NOT NULL,
  CONSTRAINT clients_user_id_fkey
    FOREIGN KEY (user_id)
    REFERENCES users (id)
    ON DELETE CASCADE,
	CONSTRAINT clients_client_secret_check CHECK (
		(is_public AND secret IS NULL) OR
		(NOT is_public AND secret IS NOT NULL)
	)
);

