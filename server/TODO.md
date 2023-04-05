# SERVER --- TODO

## Bugs

## Unimplemented

- [ ] Authentication System
    - [ ] Setup bcrypt for password salting
    - [ ] Use redis for session management
    - [ ] Determine end user Authentication System for user access

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

- [ ] Refactor server to use MVC
  - [ ] Consolidate models to dir
  - [ ] Consolidate controllers to dir
  - [ ] Consolidate "views"
  - [ ] Refactor to move logic to associated category
  - [ ] Add services

- [ ] Better separation between auth and oauth

- [ ] better redirect/errors
  - [ ] description handling

