use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RangePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Range"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-6">
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <Range size={Size::ExtraSmall} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <Range size={Size::Small} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Medium"</span>
                        <Range size={Size::Medium} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <Range size={Size::Large} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Large"</span>
                        <Range size={Size::ExtraLarge} value={50} />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="space-y-6">
                    <div>
                        <span class="text-sm text-base-content/70">"Primary"</span>
                        <Range color={Color::Primary} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Secondary"</span>
                        <Range color={Color::Secondary} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Accent"</span>
                        <Range color={Color::Accent} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Info"</span>
                        <Range color={Color::Info} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Success"</span>
                        <Range color={Color::Success} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Warning"</span>
                        <Range color={Color::Warning} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Error"</span>
                        <Range color={Color::Error} value={50} />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"States"</h2>
                <div class="space-y-6">
                    <div>
                        <span class="text-sm text-base-content/70">"Disabled"</span>
                        <Range disabled=true value={50} />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Min/Max/Step"</h2>
                <div class="space-y-6">
                    <div>
                        <span class="text-sm text-base-content/70">"Range 0-100, step 10"</span>
                        <Range min={0} max={100} step={10} value={50} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Range 0-10, step 1"</span>
                        <Range min={0} max={10} step={1} value={5} />
                    </div>
                </div>
            </section>
        </div>
    }
}
