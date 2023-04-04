use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FormFieldContainerProps {
    pub children: Children,
    pub name: String,
    pub prompt: String,
    pub error: Option<String>,
}

#[function_component(FormFieldContainer)]
pub fn form_field_container(props: &FormFieldContainerProps) -> Html {
    let mut class_list = String::from("input-container");
    let mut error_msg = String::new();

    if let Some(error) = &props.error {
        class_list = format!("{} invalid", &class_list);
        error_msg = error.to_string();
    }

    html! {
        <div class={ class_list }>
        
            { props.children.clone() }

            <label for={props.name.clone()} class="input-hint">
                { props.prompt.clone() }
            </label>
                
            <div class="input-error">
                <p>{error_msg}</p>
            </div>
        </div>
    }
}
