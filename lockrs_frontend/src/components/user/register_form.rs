use leptos::*;
use serde::{Deserialize, Serialize};
use validify::Validify;

use crate::components::ui::button::*;
use crate::components::ui::form::*;
use crate::components::ui::input::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validify)]
pub struct RegisterFormSchema {
    #[modify(trim)]
    #[validate(length(min = 1, message = "Please enter an email", code = "INVALID_EMAIL"))]
    #[validate(email(message = "Invalid email", code = "INVALID_EMAIL"))]
    pub email: String,
    #[modify(trim)]
    #[validate(length(
        min = 1,
        message = "Please enter a password",
        code = "INVALID_PASSWORD"
    ))]
    #[validate(length(
        min = 8,
        message = "Password must be at least 8 characters long",
        code = "INVALID_PASSWORD"
    ))]
    pub password: String,
}

impl RegisterFormSchema {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[component]
pub fn UserRegisterForm(cx: Scope, #[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    let class = format!(
        "grid gap-6 w-full {}",
        if let Some(c) = class { c } else { "" }
    );

    let (email, set_email) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());

    let schema = Signal::derive(cx, move || RegisterFormSchema::new(email(), password()));

    view! { cx,
        <div id="user-register-form" class=class.clone()>
            <Form
                validator=schema
            >
                <form class="space-y-4">
                    <FormField
                        name="email"
                    >
                        <FormItem>
                            <FormLabel>Email</FormLabel>
                            <FormControl>
                                <Input
                                    id="email"
                                    placeholder="name@example.com"
                                    input_type="email"
                                    autocapitalize="none"
                                    autocomplete="email"
                                    autocorrect="off"
                                    value=email
                                    on:input=move |ev| {
                                        set_email(event_target_value(&ev));
                                    }
                                />
                            </FormControl>
                            <FormDescription>
                                This is the email that will be used for your account
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                    <FormField
                        name="password"
                    >
                        <FormItem>
                            <FormLabel>Password</FormLabel>
                            <FormControl>
                                <Input
                                    id="password"
                                    placeholder="●●●●●●●●"
                                    input_type="password"
                                    autocomplete="password"
                                    autocorrect="off"
                                    value=password
                                    on:input=move |ev| {
                                        set_password(event_target_value(&ev));
                                    }
                                />
                            </FormControl>
                            <FormDescription>
                                This is the password that will be used for your account
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                    <Button
                        on:click=move |ev| {
                            ev.prevent_default();
                        }
                    >
                        Register
                    </Button>
                </form>
            </Form>
        </div>
    }
}
