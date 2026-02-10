use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SkeletonPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Skeleton"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Skeleton"</h2>
                <div class="space-y-4">
                    <Skeleton class="h-4 w-32" />
                    <Skeleton class="h-4 w-48" />
                    <Skeleton class="h-4 w-full" />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Skeleton Text"</h2>
                <div class="space-y-2">
                    <Skeleton text=true class="h-4 w-full" />
                    <Skeleton text=true class="h-4 w-5/6" />
                    <Skeleton text=true class="h-4 w-4/6" />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Circle Skeleton"</h2>
                <div class="flex gap-4 items-center">
                    <Skeleton circle=true class="w-12 h-12" />
                    <div class="space-y-2 flex-1">
                        <Skeleton class="h-4 w-32" />
                        <Skeleton class="h-3 w-48" />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Card Skeleton"</h2>
                <Card class="bg-base-100 shadow-xl max-w-sm">
                    <CardBody>
                        <div class="flex gap-4 items-start">
                            <Skeleton circle=true class="w-16 h-16 shrink-0" />
                            <div class="space-y-2 flex-1">
                                <Skeleton class="h-5 w-32" />
                                <Skeleton class="h-3 w-full" />
                                <Skeleton class="h-3 w-5/6" />
                            </div>
                        </div>
                        <div class="mt-4 space-y-2">
                            <Skeleton class="h-3 w-full" />
                            <Skeleton class="h-3 w-full" />
                            <Skeleton class="h-3 w-4/6" />
                        </div>
                        <div class="mt-4 flex gap-2">
                            <Skeleton class="h-8 w-20" />
                            <Skeleton class="h-8 w-20" />
                        </div>
                    </CardBody>
                </Card>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Table Skeleton"</h2>
                <div class="space-y-2">
                    <div class="flex gap-4">
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                    </div>
                    <div class="flex gap-4">
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                    </div>
                    <div class="flex gap-4">
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                        <Skeleton class="h-4 w-1/4" />
                    </div>
                </div>
            </section>
        </div>
    }
}
