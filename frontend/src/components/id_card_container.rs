use yew::{function_component, html, Children, Html, Properties};

use crate::styles;

#[derive(PartialEq, Properties)]
pub struct IdCardContainerProps {
    pub children: Children,
}

#[function_component(IdCardContainer)]
pub fn id_card_container(props: &IdCardContainerProps) -> Html {
    html! {
        <div class={ styles::form_styles() } id="client-registration-page">
            <div class="container" id="form-container">
                <div id="cutout" />
                <div id="lanyard-back" />
                <div id="lanyard-front" />
                <div id="lanyard-middle" />
                <div id="lanyard-metal" />

                <img 
                    src="../img/rusty-lock.png" 
                    alt="Rusty Lock"
                    height=60px
                    width=60px
                />

                { props.children.clone() }

            </div>
        </div>
    }
}

