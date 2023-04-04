
# FRONTEND --- TODO

## Bugs

[x] Fix refcell borrow mut bug on form submit with new MVC pattern - Fixed by removing redundant rc refcell on model 

## Unimplemented

- [ ] Refactor pages to use MVC
    - [x] Move view specific internal states to live in the view, not model
    - [x] Create callback struct for controller, to debloat view calls rto render the view.
    - [x] Consolidate models to represent data, not work on a page by page basis
    - [x] Determine where to move form validation methods

- [ ] Add form validation errors
    - [x] Move validation to model, as validation is considered business logic
    - [x] Add implementation into controllers to allow for view update based on validation
    - [ ] Add error message display and conditional disable based on validation to views

- [ ] Add user details page
    - [x] Create skeleton MVC
    - [ ] Determine details to display
    - [ ] Create user details route on /server
    - [ ] Avatar / Profile Pics?

- [ ] Add client details page
    - [x] Create skeleton MVC
    - [ ] Determine details to display
    - [ ] Create client details route on /server
    - [ ] How to display registered redirect urls - separate route, or list page?

- [ ] Flesh out scope confirmation page
    - [ ] How to display registered scopes - same as client details

- [ ] Authentication System
    - [ ] Determine Authentication System for user access
    - [ ] Incorporate /login, /signup, /logout with authentication system

