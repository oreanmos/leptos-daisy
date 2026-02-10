use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn InputPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Input"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-col gap-2 max-w-xs">
                    <Input placeholder="Default".to_string() />
                    <Input color={Color::Primary} placeholder="Primary".to_string() />
                    <Input color={Color::Secondary} placeholder="Secondary".to_string() />
                    <Input color={Color::Accent} placeholder="Accent".to_string() />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-col gap-2 max-w-xs">
                    <Input size={Size::ExtraSmall} placeholder="Extra Small".to_string() />
                    <Input size={Size::Small} placeholder="Small".to_string() />
                    <Input size={Size::Medium} placeholder="Medium".to_string() />
                    <Input size={Size::Large} placeholder="Large".to_string() />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled"</h2>
                <Input disabled=true placeholder="Disabled input".to_string() />
            </section>
        </div>
    }
}
