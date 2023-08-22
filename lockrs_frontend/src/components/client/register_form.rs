use leptos::*;
use serde::{Deserialize, Serialize};
use validify::schema_validation;
use validify::{ValidationErrors, Validify};

use crate::components::ui::button::*;
use crate::components::ui::form::*;
use crate::components::ui::input::*;
use crate::components::ui::radio_group::*;
use crate::components::ui::textarea::*;

const CLIENT_TYPES: &[&str] = &["public", "private"];

#[derive(Clone, Debug, Deserialize, Serialize, Validify)]
struct RegisterFormSchema {
    #[modify(trim)]
    #[validate(length(min = 1, message = "Please enter an name", code = "INVALID_NAME"))]
    #[validate(length(
        min = 2,
        message = "Please use at lease two characters",
        code = "INVALID_NAME"
    ))]
    pub name: String,

    #[modify(trim)]
    #[validate(length(
        max = 300,
        message = "Description must be 300 characters or less",
        code = "INVALID_DESCRIPTION"
    ))]
    pub description: String,

    #[modify(trim, lowercase)]
    #[validate(is_in(
        collection = CLIENT_TYPES,
        message = "Invalid client type",
        code = "INVALID_CLIENT_TYPE"
    ))]
    pub client_type: String,

    #[modify(trim)]
    #[validate(length(min = 1, message = "Please enter a url", code = "INVALID_HOMEPAGE_URL"))]
    #[validate(url(message = "Please enter a valid url", code = "INVALID_HOMEPAGE_URL",))]
    pub homepage_url: String,

    #[modify(trim)]
    pub redirect_urls: Vec<String>,
}

#[schema_validation]
fn schema_validation(schema: &RegisterFormSchema) -> Result<(), ValidationErrors> {}

impl RegisterFormSchema {
    pub fn new(
        name: String,
        description: String,
        client_type: String,
        homepage_url: String,
        redirect_urls: Vec<String>,
    ) -> Self {
        Self {
            name,
            description,
            client_type,
            homepage_url,
            redirect_urls,
        }
    }
}

#[component]
pub fn ClientRegisterForm(
    cx: Scope,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let class = format!(
        "grid gap-6 w-full {}",
        if let Some(c) = class { c } else { "" }
    );

    let (name, set_name) = create_signal(cx, String::new());
    let (description, set_description) = create_signal(cx, String::new());
    let (client_type, set_client_type) = create_signal(cx, String::new());
    let (homepage_url, set_homepage_url) = create_signal(cx, String::new());

    let initial_urls = vec![(0, create_signal(cx, String::new()))];
    let mut next_redirect_id = 1;

    let (redirect_urls, set_redirect_urls) = create_signal(cx, initial_urls);

    let mut add_redirect_url = move || {
        let sig = create_signal(cx, String::new());

        set_redirect_urls.update(move |urls| urls.push((next_redirect_id, sig)));

        next_redirect_id += 1;
    };

    let schema = Signal::derive(cx, move || {
        RegisterFormSchema::new(
            name(),
            description(),
            client_type(),
            homepage_url(),
            redirect_urls()
                .iter()
                .map(|(_id, (url, _set_url))| url())
                .collect::<Vec<String>>(),
        )
    });

    view! { cx,
        <div id="user-register-form" class=class.clone()>
            <Form validator=schema>
                <form class="space-y-4" on:submit=|_| { /* do nothing? */ }>
                    <FormField name="name">
                        <FormItem>
                            <FormLabel>Name</FormLabel>
                            <FormControl>
                                <Input
                                    id="name"
                                    placeholder="lockrs"
                                    input_type="text"
                                    autocapitalize="none"
                                    autocorrect="off"
                                    value=name
                                    on:input=move |ev| {
                                        set_name(event_target_value(&ev));
                                    }
                                />
                            </FormControl>
                            <FormDescription>
                                This is the display name that will be associated with your client
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                    <FormField name="description">
                        <FormItem>
                            <FormLabel>Description</FormLabel>
                            <FormControl>
                                <Textarea
                                    id="description"
                                    placeholder="Tell us a little bit about your application"
                                    value=description
                                    on:input=move |ev| {
                                        set_description(event_target_value(&ev));
                                    }
                                />
                            </FormControl>
                            <FormDescription>
                                This description will be seen by all users authorizing with this client
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                    <FormField name="client_type">
                        <FormItem>
                            <FormLabel>Client Type</FormLabel>
                            <FormDescription>
                                Select the type of client to register
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                        <RadioGroup
                            class="grid max-w-md grid-cols-2 gap-8 pt-2"
                            value=client_type
                            default_value="public"
                            set_value=set_client_type
                        >
                            <FormItem>
                                <FormLabel class="flex flex-row space-x-2">
                                    <FormControl>
                                        <RadioGroupItem value="public" />
                                    </FormControl>
                                    <div>
                                        <div class="items-center rounded-md border-2 border-muted p-1 hover:border-accent">
                                            <div class="space-y-2 rounded-sm bg-[#ecedef] p-2">
                                                <div class="space-y-2 rounded-md bg-white p-2 shadow-sm">
                                                    <div class="h-2 w-[80px] rounded-lg bg-[#ecedef]" />
                                                    <div class="h-2 w-[100px] rounded-lg bg-[#ecedef]" />
                                                </div>
                                                <div class="flex items-center space-x-2 rounded-md bg-white p-2 shadow-sm">
                                                    <div class="h-4 w-4 rounded-full bg-[#ecedef]" />
                                                    <div class="h-2 w-[100px] rounded-lg bg-[#ecedef]" />
                                                </div>
                                                <div class="flex items-center space-x-2 rounded-md bg-white p-2 shadow-sm">
                                                    <div class="h-4 w-4 rounded-full bg-[#ecedef]" />
                                                    <div class="h-2 w-[100px] rounded-lg bg-[#ecedef]" />
                                                </div>
                                            </div>
                                        </div>
                                        <span class="block w-full p-2 text-center font-normal">
                                          Public
                                        </span>
                                    </div>
                                </FormLabel>
                            </FormItem>
                            <FormItem>
                                <FormLabel class="flex flex-row space-x-2">
                                    <FormControl>
                                        <RadioGroupItem value="private" />
                                    </FormControl>
                                    <div>
                                        <div class="items-center rounded-md border-2 border-muted bg-popover p-1 hover:bg-accent hover:text-accent-foreground">
                                            <div class="space-y-2 rounded-sm bg-slate-950 p-2">
                                                <div class="space-y-2 rounded-md bg-slate-800 p-2 shadow-sm">
                                                    <div class="h-2 w-[80px] rounded-lg bg-slate-400" />
                                                    <div class="h-2 w-[100px] rounded-lg bg-slate-400" />
                                                </div>
                                                <div class="flex items-center space-x-2 rounded-md bg-slate-800 p-2 shadow-sm">
                                                    <div class="h-4 w-4 rounded-full bg-slate-400" />
                                                    <div class="h-2 w-[100px] rounded-lg bg-slate-400" />
                                                </div>
                                                <div class="flex items-center space-x-2 rounded-md bg-slate-800 p-2 shadow-sm">
                                                    <div class="h-4 w-4 rounded-full bg-slate-400" />
                                                    <div class="h-2 w-[100px] rounded-lg bg-slate-400" />
                                                </div>
                                            </div>
                                        </div>
                                        <span class="block w-full p-2 text-center font-normal">
                                          Private
                                        </span>
                                    </div>
                                </FormLabel>
                            </FormItem>
                        </RadioGroup>
                    </FormField>
                    <FormField name="homepage_url">
                        <FormItem>
                            <FormLabel>Homepage URL</FormLabel>
                            <FormControl>
                                <Input
                                    id="homepage_url"
                                    placeholder="https://lockrs.com/"
                                    input_type="text"
                                    autocapitalize="none"
                                    autocorrect="off"
                                    value=homepage_url
                                    on:input=move |ev| {
                                        set_homepage_url(event_target_value(&ev));
                                    }
                                />
                            </FormControl>
                            <FormDescription>
                                This is the url that will link to the homepage of your application
                            </FormDescription>
                            <FormMessage />
                        </FormItem>
                    </FormField>
                    <div>
                        <ul>
                            <For
                                each=redirect_urls
                                key=|redirect| redirect.0
                                view=move|cx, (id, (url, set_url))| {
                                    view! { cx,
                                        <li>
                                            <FormField name="redirect_urls">
                                                <FormItem>
                                                    <FormLabel class=if id != 0 { "sr-only" } else { "" }>
                                                        Redirect URLs
                                                    </FormLabel>
                                                    <FormDescription class=if id != 0 { "sr-only" } else { "" }>
                                                        Register redirect urls that the authorization server will call after processing requests
                                                    </FormDescription>
                                                    <FormControl>
                                                        <Input
                                                            placeholder="https://lockrs.com/oauth2/callback"
                                                            input_type="text"
                                                            autocapitalize="none"
                                                            autocorrect="off"
                                                            value=url
                                                            on:input=move |ev| {
                                                                set_url(event_target_value(&ev));
                                                            }
                                                        />
                                                    </FormControl>
                                                    <FormMessage />
                                                </FormItem>
                                            </FormField>
                                        </li>
                                    }
                                }
                            />
                        </ul>
                        <Button
                            variant=ButtonVariant::Outline
                            size=ButtonSize::Small
                            class="mt-2".to_string()
                            on:click=move |ev| {
                                ev.prevent_default();
                                add_redirect_url();
                            }
                        >
                            Add URL
                        </Button>
                    </div>
                    <Button
                        on:click=move |ev| {
                            ev.prevent_default();
                            log::info!("{:?}", schema());
                        }
                    >
                        Sign Up
                    </Button>
                </form>
            </Form>
        </div>
    }
}
