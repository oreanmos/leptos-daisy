use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Checkbox"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <Checkbox />
                    <Checkbox color={Color::Primary} checked=true />
                    <Checkbox color={Color::Secondary} checked=true />
                    <Checkbox color={Color::Accent} checked=true />
                    <Checkbox color={Color::Success} checked=true />
                    <Checkbox color={Color::Warning} checked=true />
                    <Checkbox color={Color::Error} checked=true />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-wrap items-center gap-4">
                    <Checkbox size={Size::ExtraSmall} checked=true />
                    <Checkbox size={Size::Small} checked=true />
                    <Checkbox size={Size::Medium} checked=true />
                    <Checkbox size={Size::Large} checked=true />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled"</h2>
                <Checkbox disabled=true checked=true />
            </section>
        </div>
    }
}
