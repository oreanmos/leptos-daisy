use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Toggle"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <Toggle checked=true />
                    <Toggle color={Color::Primary} checked=true />
                    <Toggle color={Color::Secondary} checked=true />
                    <Toggle color={Color::Accent} checked=true />
                    <Toggle color={Color::Success} checked=true />
                    <Toggle color={Color::Warning} checked=true />
                    <Toggle color={Color::Error} checked=true />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex items-center gap-4">
                    <Toggle size={Size::ExtraSmall} checked=true />
                    <Toggle size={Size::Small} checked=true />
                    <Toggle size={Size::Medium} checked=true />
                    <Toggle size={Size::Large} checked=true />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled"</h2>
                <Toggle disabled=true checked=true />
            </section>
        </div>
    }
}
