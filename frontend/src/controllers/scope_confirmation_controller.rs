use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

use crate::{
    models::ScopeConfirmationModel,
    views::ScopeConfirmationView,
};

pub enum ScopeConfirmationMessage {

}

pub struct ScopeConfirmationController {
    model: Rc<RefCell<ScopeConfirmationModel>>,
}

impl Component for ScopeConfirmationController {
    type Message = ScopeConfirmationMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: Rc::new(RefCell::new(ScopeConfirmationModel::new())),
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
