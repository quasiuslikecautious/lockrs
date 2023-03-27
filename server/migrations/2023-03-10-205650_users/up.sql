-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
	id UUID PRIMARY KEY,
	email VARCHAR(256) UNIQUE NOT NULL,
	password_hash VARCHAR(256) NOT NULL,
	CONSTRAINT min_password_length CHECK (
		(LENGTH(password_hash) >= 8)
	)
);

