use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn PageSkeletonPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Page Skeleton"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A page-level loading skeleton with configurable header, card grid, and row placeholders."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Default"
                    code=r##"<PageSkeleton />"##
                >
                    <PageSkeleton />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Cards"
                    code=r##"<PageSkeleton show_cards=true cards=3 rows=3 />"##
                >
                    <PageSkeleton show_cards=true cards=3 rows=3 />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="No Header"
                    code=r##"<PageSkeleton show_header=false rows=6 />"##
                >
                    <PageSkeleton show_header=false rows=6 />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Full Page"
                    code=r##"<PageSkeleton show_cards=true cards=4 rows=8 />"##
                >
                    <PageSkeleton show_cards=true cards=4 rows=8 />
                </ComponentPreview>
            </section>
        </div>
    }
}
