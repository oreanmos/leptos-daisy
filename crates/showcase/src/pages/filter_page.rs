use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FilterPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Filter"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Filter Group"</h2>
                <Filter>
                    <input class="btn" type="radio" name="filter1" aria-label="All" checked />
                    <input class="btn" type="radio" name="filter1" aria-label="Active" />
                    <input class="btn" type="radio" name="filter1" aria-label="Inactive" />
                </Filter>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Filter with Colors"</h2>
                <Filter>
                    <input class="btn btn-primary" type="radio" name="filter2" aria-label="All" checked />
                    <input class="btn btn-secondary" type="radio" name="filter2" aria-label="Pending" />
                    <input class="btn btn-accent" type="radio" name="filter2" aria-label="Completed" />
                    <input class="btn btn-error" type="radio" name="filter2" aria-label="Cancelled" />
                </Filter>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Filter with Reset"</h2>
                <Filter reset_label="Reset">
                    <input class="btn" type="radio" name="filter3" aria-label="Option 1" />
                    <input class="btn" type="radio" name="filter3" aria-label="Option 2" />
                    <input class="btn" type="radio" name="filter3" aria-label="Option 3" />
                </Filter>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Multiple Filter Groups"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Status"</span>
                        <Filter>
                            <input class="btn btn-sm" type="radio" name="status" aria-label="All" checked />
                            <input class="btn btn-sm" type="radio" name="status" aria-label="Active" />
                            <input class="btn btn-sm" type="radio" name="status" aria-label="Archived" />
                        </Filter>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Priority"</span>
                        <Filter>
                            <input class="btn btn-sm" type="radio" name="priority" aria-label="Any" checked />
                            <input class="btn btn-sm" type="radio" name="priority" aria-label="High" />
                            <input class="btn btn-sm" type="radio" name="priority" aria-label="Medium" />
                            <input class="btn btn-sm" type="radio" name="priority" aria-label="Low" />
                        </Filter>
                    </div>
                </div>
            </section>
        </div>
    }
}
