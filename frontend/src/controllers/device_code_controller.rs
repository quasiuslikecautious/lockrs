use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    models::UserCodeModel,
    views::{DeviceCodeFormCallbacks, DeviceCodeView},
};

pub enum DeviceCodeMessage {
    UserCodeUpdated(Event),

    SubmitButtonClicked,
}

pub struct DeviceCodeController {
    model: UserCodeModel,
    form_callbacks: DeviceCodeFormCallbacks,
}

impl Component for DeviceCodeController {
    type Message = DeviceCodeMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            model: UserCodeModel::new(),
            form_callbacks: DeviceCodeFormCallbacks {
                on_submit: ctx.link().callback(|_| Self::Message::SubmitButtonClicked),
                on_user_code_change: ctx.link().callback(Self::Message::UserCodeUpdated),
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <DeviceCodeView
                model={self.model.clone()}
                form_callbacks={self.form_callbacks.clone()}
            />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UserCodeUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input
                else {
                    return false;
                };

                self.model.set_user_code(input.value());
            }
            Self::Message::SubmitButtonClicked => {
                if !self.model.validate() {
                    return false;
                }

                self.submit_form();
            }
        };

        true
    }
}

impl DeviceCodeController {
    fn submit_form(&self) {
        spawn_local({
            let body = serde_json::to_string(&self.model).unwrap();

            async move {
                Request::post("/api/v1/device/code")
                    .header("Content-Type", "application/json")
                    .body(body)
                    .send()
                    .await
                    .expect("device code request failed");
            }
        })
    }
}
