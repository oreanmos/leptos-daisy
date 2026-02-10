use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Loading"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Variants"</h2>
                <div class="flex flex-wrap gap-8 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Spinner} />
                        <span class="text-sm text-base-content/70">"Spinner"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Dots} />
                        <span class="text-sm text-base-content/70">"Dots"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Ring} />
                        <span class="text-sm text-base-content/70">"Ring"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Ball} />
                        <span class="text-sm text-base-content/70">"Ball"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Bars} />
                        <span class="text-sm text-base-content/70">"Bars"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Infinity} />
                        <span class="text-sm text-base-content/70">"Infinity"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-6">
                    <div class="flex items-center gap-4">
                        <Loading size={Size::ExtraSmall} />
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <Loading size={Size::Small} />
                        <span class="text-sm text-base-content/70">"Small"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <Loading size={Size::Medium} />
                        <span class="text-sm text-base-content/70">"Medium"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <Loading size={Size::Large} />
                        <span class="text-sm text-base-content/70">"Large"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <Loading size={Size::ExtraLarge} />
                        <span class="text-sm text-base-content/70">"Extra Large"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-8 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Primary} />
                        <span class="text-sm text-base-content/70">"Primary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Secondary} />
                        <span class="text-sm text-base-content/70">"Secondary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Accent} />
                        <span class="text-sm text-base-content/70">"Accent"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Info} />
                        <span class="text-sm text-base-content/70">"Info"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Success} />
                        <span class="text-sm text-base-content/70">"Success"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Warning} />
                        <span class="text-sm text-base-content/70">"Warning"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading color={Color::Error} />
                        <span class="text-sm text-base-content/70">"Error"</span>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Combinations"</h2>
                <div class="flex flex-wrap gap-8 items-center">
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Spinner} size={Size::Large} color={Color::Primary} />
                        <span class="text-sm text-base-content/70">"Spinner Large Primary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Dots} size={Size::ExtraLarge} color={Color::Secondary} />
                        <span class="text-sm text-base-content/70">"Dots XL Secondary"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <Loading variant={LoadingVariant::Ring} size={Size::Medium} color={Color::Accent} />
                        <span class="text-sm text-base-content/70">"Ring Medium Accent"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
