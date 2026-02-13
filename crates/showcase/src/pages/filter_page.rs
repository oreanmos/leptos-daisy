use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FilterPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Filter"</h1>

            <section>
                <ComponentPreview
                    title="Basic Filter Group"
                    code=r#"<Filter>
    <input class="btn" type="radio" name="filter" aria-label="All" checked />
    <input class="btn" type="radio" name="filter" aria-label="Active" />
    <input class="btn" type="radio" name="filter" aria-label="Inactive" />
</Filter>"#
                >
                    <Filter>
                        <input class="btn" type="radio" name="filter1" aria-label="All" checked />
                        <input class="btn" type="radio" name="filter1" aria-label="Active" />
                        <input class="btn" type="radio" name="filter1" aria-label="Inactive" />
                    </Filter>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Filter with Colors"
                    code=r#"<Filter>
    <input class="btn btn-primary" type="radio" name="filter" aria-label="All" checked />
    <input class="btn btn-secondary" type="radio" name="filter" aria-label="Pending" />
    <input class="btn btn-accent" type="radio" name="filter" aria-label="Completed" />
    <input class="btn btn-error" type="radio" name="filter" aria-label="Cancelled" />
</Filter>"#
                >
                    <Filter>
                        <input class="btn btn-primary" type="radio" name="filter2" aria-label="All" checked />
                        <input class="btn btn-secondary" type="radio" name="filter2" aria-label="Pending" />
                        <input class="btn btn-accent" type="radio" name="filter2" aria-label="Completed" />
                        <input class="btn btn-error" type="radio" name="filter2" aria-label="Cancelled" />
                    </Filter>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Filter with Reset"
                    code=r#"<Filter reset_label="Reset">
    <input class="btn" type="radio" name="filter" aria-label="Option 1" />
    <input class="btn" type="radio" name="filter" aria-label="Option 2" />
    <input class="btn" type="radio" name="filter" aria-label="Option 3" />
</Filter>"#
                >
                    <Filter reset_label="Reset">
                        <input class="btn" type="radio" name="filter3" aria-label="Option 1" />
                        <input class="btn" type="radio" name="filter3" aria-label="Option 2" />
                        <input class="btn" type="radio" name="filter3" aria-label="Option 3" />
                    </Filter>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Multiple Filter Groups"
                    code=r#"<div class="space-y-4">
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
</div>"#
                >
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
                </ComponentPreview>
            </section>
        </div>
    }
}
