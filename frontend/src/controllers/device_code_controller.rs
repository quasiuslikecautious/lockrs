use std::{cell::RefCell, rc::Rc};

use regex::Regex;
use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    models::DeviceCodeModel,
    views::DeviceCodeView,
};

pub enum DeviceCodeMessage {
    UserCodeUpdated(Event),

    SubmitButtonClicked,
}

pub struct DeviceCodeController {
    model: Rc<RefCell<DeviceCodeModel>>,
}

impl Component for DeviceCodeController {
    type Message = DeviceCodeMessage;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(DeviceCodeModel::new())),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user_code_onchange = ctx.link().callback(Self::Message::UserCodeUpdated);
        let submit_button_onclick = ctx.link().callback(|_| Self::Message::SubmitButtonClicked);

        html! {
            <DeviceCodeView
                model={self.model.clone()}
                user_code_onchange={user_code_onchange}
                submit_button_onclick={submit_button_onclick}
            />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut model = self.model.borrow_mut();

        match msg {
            Self::Message::UserCodeUpdated(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                let Some(input) = input 
                else {
                    return false;
                };

                model.form_data.user_code = input.value();

                let user_code_regex = Regex::new(r"^([b-df-hj-np-tv-xz0-9]{8})").unwrap();

                if user_code_regex.is_match(&input.value()) {
                    model.user_code_error = None;
                } else {
                    model.user_code_error = Some(String::from("Invalid user code"));
                }
            },
            Self::Message::SubmitButtonClicked => {
                if !(model.user_code_error == None) {
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
            let form_data = self.model.borrow().form_data.clone();

            async move {
                let body = serde_json::to_string(&form_data).unwrap();

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
