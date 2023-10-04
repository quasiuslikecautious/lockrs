use leptos::*;

use crate::components::ui::button::*;
use crate::components::ui::icons::cross2_icon::*;
use crate::components::ui::input::*;

use crate::components::client::data_table::*;

#[component]
pub fn DataTableToolbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex items-center justify-between">
            <div class="flex flex-1 items-center space-x-2">
                {/*<Input
                    class="h-8 w-[150px] lg:w-[250px]"
                    value=table.getColumn("title").getFilterValue | ""
                    on:change=|ev| {
                        table.getColumn("title").setFiltervalue(enevt_taregt_value(&ev));
                    }
                />*/}
                { /*foreach columns*/ }
                { /*<Show
                    when=isFiltered
                    fallback=move |cx| view! { cx,
                        <div class="hidden" />
                    }*/ }
                    <Button
                        variant=ButtonVariant::Ghost
                        class="h-8 px-2 lg:px-3".to_string()
                        on:click=|ev| {
                            log::info!("reset");
                        }
                    >
                        Reset
                        <Cross2Icon class="ml-2 h-4 w-4" />
                    </Button>
                { /*</Show>*/ }
            </div>
        </div>
    }
}
