use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TablePage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Table"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Data tables with zebra stripes, pinned rows/columns, and size variants."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "table", type_label: "base", description: "Base class for table" },
                    ClassEntry { name: "table-zebra", type_label: "modifier", description: "Zebra striping for rows" },
                    ClassEntry { name: "table-pin-rows", type_label: "modifier", description: "Pin header and footer rows" },
                    ClassEntry { name: "table-pin-cols", type_label: "modifier", description: "Pin first column" },
                    ClassEntry { name: "table-lg", type_label: "size", description: "Large size" },
                    ClassEntry { name: "table-md", type_label: "size", description: "Medium size (default)" },
                    ClassEntry { name: "table-sm", type_label: "size", description: "Small size" },
                    ClassEntry { name: "table-xs", type_label: "size", description: "Extra small size" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Table"
                    code=r##"<Table>
    <thead>
        <tr>
            <th>"#"</th>
            <th>"Name"</th>
            <th>"Job"</th>
            <th>"Favorite Color"</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <th>"1"</th>
            <td>"Cy Ganderton"</td>
            <td>"Quality Control"</td>
            <td>"Blue"</td>
        </tr>
        <tr>
            <td>"2"</td>
            <td>"Hart Hagerty"</td>
            <td>"Desktop Support"</td>
            <td>"Purple"</td>
        </tr>
        <tr>
            <td>"3"</td>
            <td>"Brice Swyre"</td>
            <td>"Tax Accountant"</td>
            <td>"Red"</td>
        </tr>
    </tbody>
</Table>"##
                >
                    <div class="overflow-x-auto">
                        <Table>
                            <thead>
                                <tr>
                                    <th>"#"</th>
                                    <th>"Name"</th>
                                    <th>"Job"</th>
                                    <th>"Favorite Color"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <th>"1"</th>
                                    <td>"Cy Ganderton"</td>
                                    <td>"Quality Control"</td>
                                    <td>"Blue"</td>
                                </tr>
                                <tr>
                                    <td>"2"</td>
                                    <td>"Hart Hagerty"</td>
                                    <td>"Desktop Support"</td>
                                    <td>"Purple"</td>
                                </tr>
                                <tr>
                                    <td>"3"</td>
                                    <td>"Brice Swyre"</td>
                                    <td>"Tax Accountant"</td>
                                    <td>"Red"</td>
                                </tr>
                            </tbody>
                        </Table>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Zebra Striped"
                    code=r##"<Table zebra=true>
    <thead>
        <tr>
            <th>"#"</th>
            <th>"Name"</th>
            <th>"Job"</th>
        </tr>
    </thead>
    <tbody>
        <tr><th>"1"</th><td>"Cy Ganderton"</td><td>"QC"</td></tr>
        <tr><th>"2"</th><td>"Hart Hagerty"</td><td>"Support"</td></tr>
        <tr><th>"3"</th><td>"Brice Swyre"</td><td>"Tax"</td></tr>
    </tbody>
</Table>"##
                >
                    <div class="overflow-x-auto">
                        <Table zebra=true>
                            <thead>
                                <tr>
                                    <th>"#"</th>
                                    <th>"Name"</th>
                                    <th>"Job"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr><th>"1"</th><td>"Cy Ganderton"</td><td>"QC"</td></tr>
                                <tr><th>"2"</th><td>"Hart Hagerty"</td><td>"Support"</td></tr>
                                <tr><th>"3"</th><td>"Brice Swyre"</td><td>"Tax"</td></tr>
                            </tbody>
                        </Table>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Pin Rows & Columns"
                    code=r##"<Table pin_rows=true pin_cols=true>
    <thead>
        <tr>
            <th>"#"</th>
            <th>"Name"</th>
            <th>"Job"</th>
        </tr>
    </thead>
    <tbody>
        <tr><th>"1"</th><td>"Cy"</td><td>"QC"</td></tr>
        <tr><th>"2"</th><td>"Hart"</td><td>"Support"</td></tr>
        <tr><th>"3"</th><td>"Brice"</td><td>"Tax"</td></tr>
        <tr><th>"4"</th><td>"Marjy"</td><td>"Intern"</td></tr>
        <tr><th>"5"</th><td>"Yancy"</td><td>"Manager"</td></tr>
    </tbody>
</Table>"##
                >
                    <div class="overflow-x-auto max-h-48">
                        <Table pin_rows=true pin_cols=true>
                            <thead>
                                <tr>
                                    <th>"#"</th>
                                    <th>"Name"</th>
                                    <th>"Job"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr><th>"1"</th><td>"Cy"</td><td>"QC"</td></tr>
                                <tr><th>"2"</th><td>"Hart"</td><td>"Support"</td></tr>
                                <tr><th>"3"</th><td>"Brice"</td><td>"Tax"</td></tr>
                                <tr><th>"4"</th><td>"Marjy"</td><td>"Intern"</td></tr>
                                <tr><th>"5"</th><td>"Yancy"</td><td>"Manager"</td></tr>
                            </tbody>
                        </Table>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Table size={Size::ExtraSmall}>
    <thead><tr><th>"XS"</th><td>"Extra Small"</td></tr></thead>
</Table>

<Table size={Size::Small}>
    <thead><tr><th>"SM"</th><td>"Small"</td></tr></thead>
</Table>

<Table size={Size::Medium}>
    <thead><tr><th>"MD"</th><td>"Medium"</td></tr></thead>
</Table>

<Table size={Size::Large}>
    <thead><tr><th>"LG"</th><td>"Large"</td></tr></thead>
</Table>"##
                >
                    <div class="space-y-4">
                        <div class="overflow-x-auto">
                            <Table size=Size::ExtraSmall>
                                <thead><tr><th>"XS"</th><td>"Extra Small"</td></tr></thead>
                            </Table>
                        </div>
                        <div class="overflow-x-auto">
                            <Table size=Size::Small>
                                <thead><tr><th>"SM"</th><td>"Small"</td></tr></thead>
                            </Table>
                        </div>
                        <div class="overflow-x-auto">
                            <Table size=Size::Medium>
                                <thead><tr><th>"MD"</th><td>"Medium"</td></tr></thead>
                            </Table>
                        </div>
                        <div class="overflow-x-auto">
                            <Table size=Size::Large>
                                <thead><tr><th>"LG"</th><td>"Large"</td></tr></thead>
                            </Table>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
