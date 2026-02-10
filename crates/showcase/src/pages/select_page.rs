use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Select"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-col gap-2 max-w-xs">
                    <Select>
                        <option disabled=true selected=true>"Pick a color"</option>
                        <option>"Red"</option>
                        <option>"Blue"</option>
                        <option>"Green"</option>
                    </Select>
                    <Select color={Color::Primary}>
                        <option>"Primary Select"</option>
                        <option>"Option A"</option>
                        <option>"Option B"</option>
                    </Select>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-col gap-2 max-w-xs">
                    <Select size={Size::Small}>
                        <option>"Small"</option>
                    </Select>
                    <Select size={Size::Large}>
                        <option>"Large"</option>
                    </Select>
                </div>
            </section>
        </div>
    }
}
