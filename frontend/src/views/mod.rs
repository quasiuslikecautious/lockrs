mod client_registration_view;
mod device_code_view;
mod login_view;
mod logout_view;
mod logout_success_view;
mod scope_confirmation_view;
mod signup_view;

pub use self::{
    client_registration_view::*,
    device_code_view::*,
    login_view::*,
    logout_view::*,
    logout_success_view::*,
    scope_confirmation_view::*,
    signup_view::*,
};
