# SERVER --- TODO

## Bugs

- [ ] session_controller -> create: should return a session response, not a session model

- [ ] in token_controller, scopes should be an option as it is not required for refresh_token

- [x] there is not a device authorization model

- [x] there is not an authorization code model

## Improvements

- [ ] Authentication System
    - [x] Setup bcrypt for password salting
    - [ ] Use redis for session management
    - [x] Determine end user Authentication System for user access

- [ ] Use JWTs
  - [ ] Access Tokens
  - [ ] Redirect Tokens

- [ ] Add caching

- [ ] Add filtering to verify JWT before accessing DB

- [x] scope handling

- [x] auth code generation

- [x] device code
  - [x] generation
  - [x] user code generation
  - [x] verification uri

- [ ] middlewares
  - [ ] HTTPS server
  - [ ] HTTPS online requests
  - [ ] Logging
  - [ ] Rate Limiting

- [x] add PKCE support to auth code flow

- [x] client registration

- [ ] Add better db types
    - [ ] DbUrl - parses string from db to url, converts to_string to db
    - [ ] Db<Scopes|Vec> - maps Vec from db to Vec that contains types, not options

- [x] Refactor server to use MVC
  - [x] Consolidate models to dir
  - [x] Consolidate controllers to dir
  - [x] Consolidate "views"
  - [x] Refactor to move logic to associated category
  - [x] Add services

- [x] Better separation between auth and oauth

- [x] better redirect/errors
  - [x] description handling

- [ ] Refactor models
    - [x] Add models to name of all models
    - [x] Fix mappers to not use responses/move response to model
        - [x] device_authorization
    - [x] Move all requests (mostly from /auth) into models
    - [x] Create separatation of models

- [ ] Fix mappers
    - [ ] move mappers to correct domain dirs instead of all in shared
    - [ ] determine how model -> response mapping should be handled

- [x] client_auth_service is only used by /oauth2

