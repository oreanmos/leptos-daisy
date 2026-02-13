use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SkeletonPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Skeleton"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Loading placeholders that show where content will appear."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r##"<div class="flex flex-col gap-4 w-52">
    <Skeleton class="h-32 w-full" />
    <Skeleton class="h-4 w-28" />
    <Skeleton class="h-4 w-full" />
    <Skeleton class="h-4 w-full" />
</div>"##
                >
                    <div class="flex flex-col gap-4 w-52">
                        <Skeleton class="h-32 w-full" />
                        <Skeleton class="h-4 w-28" />
                        <Skeleton class="h-4 w-full" />
                        <Skeleton class="h-4 w-full" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Circle"
                    code=r##"<Skeleton circle=true class="w-16 h-16" />"##
                >
                    <Skeleton circle=true class="w-16 h-16" />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Card Skeleton"
                    code=r##"<div class="flex flex-col gap-4 w-52">
    <Skeleton class="h-32 w-full" />
    <Skeleton class="h-4 w-28" />
    <Skeleton class="h-4 w-full" />
</div>"##
                >
                    <div class="flex flex-col gap-4 w-52">
                        <Skeleton class="h-32 w-full" />
                        <Skeleton class="h-4 w-28" />
                        <Skeleton class="h-4 w-full" />
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
