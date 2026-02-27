use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SortControllerPage() -> impl IntoView {
    let sort = use_sort();

    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Sort Controller"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Reactive sort state for tables. Click column headers to toggle sort direction."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sortable Table"
                    code=r#"let sort = use_sort();
<Table>
    <TableHead>
        <TableRow>
            <SortableHeaderCell controller=sort field="name">"Name"</SortableHeaderCell>
            <SortableHeaderCell controller=sort field="role">"Role"</SortableHeaderCell>
        </TableRow>
    </TableHead>
</Table>"#
                >
                    <div class="overflow-x-auto">
                        <Table>
                            <TableHead>
                                <TableRow>
                                    <SortableHeaderCell controller=sort field="name">"Name"</SortableHeaderCell>
                                    <SortableHeaderCell controller=sort field="role">"Role"</SortableHeaderCell>
                                    <SortableHeaderCell controller=sort field="email">"Email"</SortableHeaderCell>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                <TableRow hover=true>
                                    <TableCell>"Alice"</TableCell>
                                    <TableCell>"Engineer"</TableCell>
                                    <TableCell>"alice@example.com"</TableCell>
                                </TableRow>
                                <TableRow hover=true>
                                    <TableCell>"Bob"</TableCell>
                                    <TableCell>"Designer"</TableCell>
                                    <TableCell>"bob@example.com"</TableCell>
                                </TableRow>
                                <TableRow hover=true>
                                    <TableCell>"Carol"</TableCell>
                                    <TableCell>"Manager"</TableCell>
                                    <TableCell>"carol@example.com"</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </div>
                    <div class="mt-4 text-sm text-base-content/60">
                        <p>{move || format!("Sort column: {:?}", sort.column())}</p>
                        <p>{move || format!("Ascending: {}", sort.is_ascending())}</p>
                        <p>{move || format!("Sort param: {:?}", sort.sort_param())}</p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
