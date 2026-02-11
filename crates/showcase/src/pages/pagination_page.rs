use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn PaginationPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Pagination"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Pagination>
                    <PaginationItem>"«"</PaginationItem>
                    <PaginationItem>"1"</PaginationItem>
                    <PaginationItem active=true>"2"</PaginationItem>
                    <PaginationItem>"3"</PaginationItem>
                    <PaginationItem>"»"</PaginationItem>
                </Pagination>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled"</h2>
                <Pagination>
                    <PaginationItem disabled=true>"«"</PaginationItem>
                    <PaginationItem active=true>"1"</PaginationItem>
                    <PaginationItem>"2"</PaginationItem>
                    <PaginationItem>"»"</PaginationItem>
                </Pagination>
            </section>
        </div>
    }
}
