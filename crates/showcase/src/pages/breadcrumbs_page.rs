use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn BreadcrumbsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Breadcrumbs"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Breadcrumbs"</h2>
                <Breadcrumbs items={vec![
                    BreadcrumbItem::new("Home", "/"),
                    BreadcrumbItem::new("Documents", "/documents"),
                    BreadcrumbItem::current("Current Page"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Long Breadcrumbs"</h2>
                <Breadcrumbs items={vec![
                    BreadcrumbItem::new("Home", "/"),
                    BreadcrumbItem::new("Products", "/products"),
                    BreadcrumbItem::new("Electronics", "/products/electronics"),
                    BreadcrumbItem::new("Computers", "/products/electronics/computers"),
                    BreadcrumbItem::current("Laptops"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Short Breadcrumbs"</h2>
                <Breadcrumbs items={vec![
                    BreadcrumbItem::new("Home", "/"),
                    BreadcrumbItem::current("Dashboard"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Custom Styling"</h2>
                <Breadcrumbs
                    items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Settings", "/settings"),
                        BreadcrumbItem::current("Profile"),
                    ]}
                    class="bg-base-200 p-2 rounded-lg"
                />
            </section>
        </div>
    }
}
