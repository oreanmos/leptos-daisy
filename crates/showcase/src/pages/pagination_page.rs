use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn PaginationPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Pagination"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Page navigation controls with size variants and active/disabled states."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "join", type_label: "base", description: "Container for pagination group" },
                    ClassEntry { name: "btn", type_label: "base", description: "Button for pagination items" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r##"<Pagination>
    <PaginationItem>"«"</PaginationItem>
    <PaginationItem>"1"</PaginationItem>
    <PaginationItem active=true>"2"</PaginationItem>
    <PaginationItem>"3"</PaginationItem>
    <PaginationItem>"»"</PaginationItem>
</Pagination>"##
                >
                    <Pagination>
                        <PaginationItem>"«"</PaginationItem>
                        <PaginationItem>"1"</PaginationItem>
                        <PaginationItem active=true>"2"</PaginationItem>
                        <PaginationItem>"3"</PaginationItem>
                        <PaginationItem>"»"</PaginationItem>
                    </Pagination>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Pagination size=Size::Small>
    <PaginationItem>"1"</PaginationItem>
    <PaginationItem active=true>"2"</PaginationItem>
    <PaginationItem>"3"</PaginationItem>
</Pagination>

<Pagination size=Size::Medium>
    <PaginationItem>"1"</PaginationItem>
    <PaginationItem active=true>"2"</PaginationItem>
    <PaginationItem>"3"</PaginationItem>
</Pagination>

<Pagination size=Size::Large>
    <PaginationItem>"1"</PaginationItem>
    <PaginationItem active=true>"2"</PaginationItem>
    <PaginationItem>"3"</PaginationItem>
</Pagination>"##
                >
                    <div class="space-y-4">
                        <Pagination size=Size::Small>
                            <PaginationItem>"1"</PaginationItem>
                            <PaginationItem active=true>"2"</PaginationItem>
                            <PaginationItem>"3"</PaginationItem>
                        </Pagination>
                        <Pagination size=Size::Medium>
                            <PaginationItem>"1"</PaginationItem>
                            <PaginationItem active=true>"2"</PaginationItem>
                            <PaginationItem>"3"</PaginationItem>
                        </Pagination>
                        <Pagination size=Size::Large>
                            <PaginationItem>"1"</PaginationItem>
                            <PaginationItem active=true>"2"</PaginationItem>
                            <PaginationItem>"3"</PaginationItem>
                        </Pagination>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Disabled"
                    code=r##"<Pagination>
    <PaginationItem disabled=true>"«"</PaginationItem>
    <PaginationItem active=true>"1"</PaginationItem>
    <PaginationItem>"2"</PaginationItem>
    <PaginationItem>"»"</PaginationItem>
</Pagination>"##
                >
                    <Pagination>
                        <PaginationItem disabled=true>"«"</PaginationItem>
                        <PaginationItem active=true>"1"</PaginationItem>
                        <PaginationItem>"2"</PaginationItem>
                        <PaginationItem>"»"</PaginationItem>
                    </Pagination>
                </ComponentPreview>
            </section>
        </div>
    }
}
