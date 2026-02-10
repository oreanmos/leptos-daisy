use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TextareaPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Textarea"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-col gap-2 max-w-xs">
                    <Textarea placeholder="Default".to_string() />
                    <Textarea color={Color::Primary} placeholder="Primary".to_string() />
                    <Textarea color={Color::Secondary} placeholder="Secondary".to_string() />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled"</h2>
                <Textarea disabled=true placeholder="Disabled".to_string() />
            </section>
        </div>
    }
}
