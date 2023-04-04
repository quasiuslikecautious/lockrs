use yew::prelude::*;

use crate::{
    models::ScopeConfirmationModel,
    views::ScopeConfirmationView,
};

pub enum ScopeConfirmationMessage {

}

pub struct ScopeConfirmationController {
    model: ScopeConfirmationModel,
}

impl Component for ScopeConfirmationController {
    type Message = ScopeConfirmationMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: ScopeConfirmationModel::new(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ScopeConfirmationView
                model={self.model.clone()}
            />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
