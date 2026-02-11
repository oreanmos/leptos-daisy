use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SkeletonPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Skeleton"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <div class="flex flex-col gap-4 w-52">
                    <Skeleton class="h-32 w-full" />
                    <Skeleton class="h-4 w-28" />
                    <Skeleton class="h-4 w-full" />
                    <Skeleton class="h-4 w-full" />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Circle"</h2>
                <Skeleton circle=true class="w-16 h-16" />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Card Skeleton"</h2>
                <div class="flex flex-col gap-4 w-52">
                    <Skeleton class="h-32 w-full" />
                    <Skeleton class="h-4 w-28" />
                    <Skeleton class="h-4 w-full" />
                </div>
            </section>
        </div>
    }
}
