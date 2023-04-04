#FRONTEND --- TODO#

##Bugs##
[x] Fix refcell borrow mut bug on form submit with new MVC pattern - Fixed by removing redundant rc refcell on model 

##Unimplemented##
[ ] Refactor pages to use MVC
    [ ] Move view specific internal states to live in the view, not model
    [ ] Determine where to move form validation methods
    [ ] Create callback struct for controller, to debloat view calls rto render the view.

[ ] Add form validation errors

[ ] Add user details page
    [ ] Determine details to display
    [ ] Create user details route on /server
    [ ] Avatar / Profile Pics?

[ ] Add client details page
    [ ] Determine details to display
    [ ] Create client details route on /server
    [ ] How to display registered redirect urls - separate route, or list page?
    [ ] How to display registered scopes - same as above

[ ] Flesh out scope confirmation page

[ ] Authentication System
    [ ] Determine Authentication System for user access

