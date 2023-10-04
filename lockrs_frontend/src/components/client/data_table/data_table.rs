use leptos::*;

use crate::components::ui::table::*;

use super::data_table_pagination::*;
use super::data_table_toolbar::*;

#[component]
pub fn DataTable(
    cx: Scope,
    // columns: ,
) -> impl IntoView {
    view! { cx,
        <div class="space-y-4">
            <DataTableToolbar { /*table=table*/ } />
            <div class="rounded-md border">
                <Table>
                    <TableHeader>
                        { /*<For each=headerGroup key=headerGroup.id view={*/ }
                            <TableRow>
                                { /*<For each=headerGroup.headers key=header.id view={*/ }
                                    <TableHead>
                                        { /*header.isPlaceholder ? null : flexRender(header.column.columnDef.header, header.getContext()) */ }
                                        <div />
                                    </TableHead>
                                { /*}/>*/ }
                            </TableRow>
                        { /*}/>*/ }
                    </TableHeader>
                    <TableBody>
                        { /*<Show when={table.getRowModel().rows.length > 0}*/ }
                        { /*fallback=move |cx| view! { */ }
                            { /*<For each=table.getRowModel().rows key=row.id view={*/ }
                                <TableRow data_state="selected"> { /*row.is_selected*/ }
                                    { /*<For each=row.getVisibleCells key=cell.id view={*/ }
                                        <TableCell>
                                            { /*flexRender(cell.column.columnDef.cell, cell.getContext())*/ }
                                            <div />
                                        </TableCell>
                                    { /*}/>*/ }
                                </TableRow>
                            { /*/>*/ }
                        { /*}>*/ }
                            <TableRow>
                                <TableCell class="h-24 text-center".to_string()>
                                    No results.
                                </TableCell>
                            </TableRow>
                        { /*</Show>*/ }
                    </TableBody>
                </Table>
            </div>
            { /*<DataTablePagination table=table*/ }
        </div>
    }
}
