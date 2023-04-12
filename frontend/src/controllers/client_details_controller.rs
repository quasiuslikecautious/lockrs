use yew::prelude::*;

use crate::{models::ClientModel, views::ClientDetailsView};

pub enum ClientDetailsMessage {}

#[derive(Clone, PartialEq, Properties)]
pub struct ClientDetailsControllerProps {
    pub client_id: String,
}

pub struct ClientDetailsController {
    model: ClientModel,
}

impl Component for ClientDetailsController {
    type Message = ClientDetailsMessage;
    type Properties = ClientDetailsControllerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            model: ClientModel::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <ClientDetailsView
                    model={self.model.clone()}
                    client_id={ctx.props().client_id.clone()}
                />
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
