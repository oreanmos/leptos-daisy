use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TablePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Table"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Table"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Zebra Striped"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Pin Rows & Columns"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
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
            </section>
        </div>
    }
}
