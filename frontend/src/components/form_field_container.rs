use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FormFieldContainerProps {
    pub children: Children,
    pub name: String,
    pub prompt: String,
}

#[function_component(FormFieldContainer)]
pub fn form_field_container(props: &FormFieldContainerProps) -> Html {
    html! {
        <div class="input-container">
        
            { props.children.clone() }

            <label for={props.name.clone()} class="input-hint">
                { props.prompt.clone() }
            </label>
        </div>
    }
}
