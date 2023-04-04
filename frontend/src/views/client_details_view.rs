use yew::prelude::*;

use crate::models::ClientModel;

#[derive(Clone, PartialEq, Properties)]
pub struct ClientDetailsViewProps {
    pub model: ClientModel,
    pub client_id: String,
}

pub struct ClientDetailsView;

impl Component for ClientDetailsView {
    type Message = ();
    type Properties = ClientDetailsViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ctx.props().client_id.clone()}</h2>
            </>
        }
    }
}
