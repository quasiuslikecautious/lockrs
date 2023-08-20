use leptos::*;
use uuid::Uuid;
use validify::{ValidationError, ValidationErrors, Validify};
use wasm_bindgen::UnwrapThrowExt;

use crate::components::ui::label::*;

fn use_id() -> String {
    Uuid::new_v4().to_string()
}

fn is_field_error_present(name: &'static str, errors: Option<ValidationErrors>) -> bool {
    let Some(errors) = errors else {
        return false;
    };

    errors.field_errors().iter().any(|e| match e {
        ValidationError::Field { field, .. } => *field == name,
        _ => false,
    })
}

fn extract_field_error_message(
    name: &'static str,
    errors: Option<ValidationErrors>,
) -> Option<String> {
    let Some(errors) = errors else {
        return None;
    };

    errors.field_errors().iter().find_map(|e| match e {
        ValidationError::Field { field, message, .. } => {
            if *field == name {
                message.as_ref().cloned()
            } else {
                None
            }
        }
        _ => None,
    })
}

#[derive(Clone)]
pub struct FormContext {
    errors: Signal<Option<ValidationErrors>>,
}

#[component]
pub fn Form<T>(cx: Scope, validator: Signal<T>, children: Children) -> impl IntoView
where
    T: Validify + Clone + 'static,
{
    let errors = Signal::derive(cx, move || match validator.get().validify_self() {
        Ok(()) => None,
        Err(e) => Some(e),
    });

    let context = FormContext { errors };

    provide_context(cx, context);

    view! { cx,
        <>
            {children(cx)}
        </>
    }
}

#[derive(Clone)]
struct FormFieldContext {
    name: &'static str,
    dirty: ReadSignal<bool>,
}

#[component]
pub fn FormField(cx: Scope, name: &'static str, children: Children) -> impl IntoView {
    let (dirty, set_dirty) = create_signal(cx, false);
    let context = FormFieldContext { name, dirty };
    provide_context(cx, context);

    view! { cx,
        <div
            on:input=move |_| {
                set_dirty(true);
            }
        >
            {children(cx)}
        </div>
    }
}

struct UseFormFieldReturn {
    id: String,
    name: &'static str,
    form_item_id: String,
    form_description_id: String,
    form_message_id: String,
    dirty: ReadSignal<bool>,
    errors: Signal<Option<ValidationErrors>>,
}

fn use_form_field(cx: Scope) -> UseFormFieldReturn {
    let field_context = use_context::<FormFieldContext>(cx)
        .expect_throw("use_form_field should be used within <FormField>");
    let item_context = use_context::<FormItemContext>(cx)
        .expect_throw("use_form_field should be used within <FormItem>");
    let form_context =
        use_context::<FormContext>(cx).expect_throw("use_form_field should be used within <Form>");

    let id = item_context.id;

    UseFormFieldReturn {
        id: id.clone(),
        name: field_context.name,
        form_item_id: format!("{}-form-item", &id),
        form_description_id: format!("{}-form-item-description", &id),
        form_message_id: format!("{}-form-item-message", &id),
        dirty: field_context.dirty,
        errors: form_context.errors,
    }
}

#[derive(Clone)]
pub struct FormItemContext {
    id: String,
}

#[component]
pub fn FormItem(cx: Scope, children: Children) -> impl IntoView {
    let id = use_id();
    let context = FormItemContext { id: id.clone() };
    provide_context(cx, context);

    view! { cx,
        <div id=id class="space-y-2">
            {children(cx)}
        </div>
    }
}

#[component]
pub fn FormLabel(cx: Scope, children: Children) -> impl IntoView {
    let context = use_form_field(cx);
    let is_error = Signal::derive(cx, move || {
        (context.dirty)() && is_field_error_present(context.name, (context.errors)())
    });
    let error_class = Signal::derive(cx, move || {
        if is_error() {
            "text-destructive".to_string()
        } else {
            String::new()
        }
    });

    view! { cx,
        <Label
            class_signal=error_class
            html_for=context.form_item_id
        >
            {children(cx)}
        </Label>
    }
}

#[component]
pub fn FormControl(cx: Scope, children: Children) -> impl IntoView {
    let context = use_form_field(cx);
    let is_error =
        move || (context.dirty)() && is_field_error_present(context.name, (context.errors)());
    let described_by = move || {
        if is_error() {
            context.form_description_id.clone()
        } else {
            format!(
                "{} {}",
                context.form_description_id, context.form_message_id
            )
        }
    };

    view! { cx,
        <div
            id=context.form_item_id
            aria-describedby=described_by
            aria-invalid=is_error
        >
            {children(cx)}
        </div>
    }
}

#[component]
pub fn FormDescription(cx: Scope, children: Children) -> impl IntoView {
    let context = use_form_field(cx);

    view! { cx,
        <p
            id=context.form_description_id
            class="text-[0.8rem] text-muted-foreground"
        >
            {children(cx)}
        </p>
    }
}

#[component]
pub fn FormMessage(cx: Scope) -> impl IntoView {
    let context = use_form_field(cx);
    let is_error =
        move || (context.dirty)() && is_field_error_present(context.name, (context.errors)());
    let error_message = move || extract_field_error_message(context.name, (context.errors)());

    view! { cx,
        <Show
            when=is_error
            fallback=|_cx| view! { cx, "" }
        >
            <p
                id=context.form_message_id.clone()
                class="text-sm font-medium text-destructive"
            >
                {error_message()}
            </p>
        </Show>
    }
}
