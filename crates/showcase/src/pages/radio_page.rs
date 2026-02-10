use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RadioPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Radio"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-col gap-2">
                    <Radio name="color-demo" value="1".to_string() color={Color::Primary} checked=true />
                    <Radio name="color-demo" value="2".to_string() color={Color::Secondary} />
                    <Radio name="color-demo" value="3".to_string() color={Color::Accent} />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex items-center gap-4">
                    <Radio name="size-demo" value="xs".to_string() size={Size::ExtraSmall} checked=true />
                    <Radio name="size-demo" value="sm".to_string() size={Size::Small} />
                    <Radio name="size-demo" value="md".to_string() size={Size::Medium} />
                    <Radio name="size-demo" value="lg".to_string() size={Size::Large} />
                </div>
            </section>
        </div>
    }
}
