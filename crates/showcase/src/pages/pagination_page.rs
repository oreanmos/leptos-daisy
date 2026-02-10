use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn PaginationPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Pagination"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Pagination"</h2>
                <Pagination
                    current_page={1usize}
                    total_pages={10usize}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Navigation"</h2>
                <Pagination
                    current_page={5usize}
                    total_pages={10usize}
                    show_prev_next={true}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With First/Last"</h2>
                <Pagination
                    current_page={5usize}
                    total_pages={20usize}
                    show_first_last={true}
                    show_prev_next={true}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Limited Visible Pages"</h2>
                <Pagination
                    current_page={10usize}
                    total_pages={50usize}
                    max_visible={5usize}
                    show_prev_next={true}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <Pagination
                            current_page={3usize}
                            total_pages={10usize}
                            size={Size::ExtraSmall}
                        />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <Pagination
                            current_page={3usize}
                            total_pages={10usize}
                            size={Size::Small}
                        />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <Pagination
                            current_page={3usize}
                            total_pages={10usize}
                            size={Size::Large}
                        />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Custom Labels"</h2>
                <Pagination
                    current_page={3usize}
                    total_pages={10usize}
                    show_prev_next={true}
                    prev_label={"Previous"}
                    next_label={"Next"}
                />
            </section>
        </div>
    }
}
