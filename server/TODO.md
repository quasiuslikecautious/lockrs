# SERVER --- TODO

## Bugs

## Unimplemented

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

- [ ] client registration



## Improvements

- [x] Refactor server to use MVC
  - [x] Consolidate models to dir
  - [x] Consolidate controllers to dir
  - [x] Consolidate "views"
  - [x] Refactor to move logic to associated category
  - [x] Add services

- [x] Better separation between auth and oauth

- [x] better redirect/errors
  - [x] description handling

