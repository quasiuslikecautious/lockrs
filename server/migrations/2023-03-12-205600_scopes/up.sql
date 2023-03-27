-- Your SQL goes here
CREATE TABLE IF NOT EXISTS scopes (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  description TEXT NOT NULL,
  client_id VARCHAR(32),
  CONSTRAINT scopes_client_id_fkey
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
    ON DELETE CASCADE
);

CREATE INDEX scopes_name_idx ON scopes (name);

INSERT INTO scopes (name, description)
VALUES 
  ('read', 'Allows the client application to read user data, such as their profile information or email address.'),
  ('write', 'Allows the client application to create or modify user data, such as adding or editing user comments.'),
  ('delete', 'Allows the client application to delete user data, such as removing user comments.'),
  ('offline_access', 'Allows the client application to access the user''s resources even when the user is not actively logged in.'),
  ('openid', 'Allows the client application to obtain the user''s OpenID identifier, which can be used to authenticate the user on other systems.'),
  ('profile', 'Allows the client application to obtain the user''s profile information, such as their name and profile picture.');

