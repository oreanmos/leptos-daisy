use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn BreadcrumbsPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Breadcrumbs"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Navigation trails showing the current page location within the site hierarchy."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "breadcrumbs", type_label: "base", description: "Base class for breadcrumbs container" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Breadcrumbs"
                    code=r##"<Breadcrumbs items={vec![
    BreadcrumbItem::new("Home", "/"),
    BreadcrumbItem::new("Documents", "/documents"),
    BreadcrumbItem::current("Current Page"),
]} />"##
                >
                    <Breadcrumbs items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Documents", "/documents"),
                        BreadcrumbItem::current("Current Page"),
                    ]} />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Long Breadcrumbs"
                    code=r##"<Breadcrumbs items={vec![
    BreadcrumbItem::new("Home", "/"),
    BreadcrumbItem::new("Products", "/products"),
    BreadcrumbItem::new("Electronics", "/products/electronics"),
    BreadcrumbItem::new("Computers", "/products/electronics/computers"),
    BreadcrumbItem::current("Laptops"),
]} />"##
                >
                    <Breadcrumbs items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::new("Products", "/products"),
                        BreadcrumbItem::new("Electronics", "/products/electronics"),
                        BreadcrumbItem::new("Computers", "/products/electronics/computers"),
                        BreadcrumbItem::current("Laptops"),
                    ]} />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Short Breadcrumbs"
                    code=r##"<Breadcrumbs items={vec![
    BreadcrumbItem::new("Home", "/"),
    BreadcrumbItem::current("Dashboard"),
]} />"##
                >
                    <Breadcrumbs items={vec![
                        BreadcrumbItem::new("Home", "/"),
                        BreadcrumbItem::current("Dashboard"),
                    ]} />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Custom Styling"
                    code=r##"<Breadcrumbs
    items={vec![
        BreadcrumbItem::new("Home", "/"),
        BreadcrumbItem::new("Settings", "/settings"),
        BreadcrumbItem::current("Profile"),
    ]}
    class="bg-base-200 p-2 rounded-lg"
/>"##
                >
                    <Breadcrumbs
                        items={vec![
                            BreadcrumbItem::new("Home", "/"),
                            BreadcrumbItem::new("Settings", "/settings"),
                            BreadcrumbItem::current("Profile"),
                        ]}
                        class="bg-base-200 p-2 rounded-lg"
                    />
                </ComponentPreview>
            </section>
        </div>
    }
}
