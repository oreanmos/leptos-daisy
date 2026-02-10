use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RatingPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Rating"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Rating value=3 />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-2">
                    <Rating size={Size::ExtraSmall} value=2 name="size-xs".to_string() />
                    <Rating size={Size::Small} value=3 name="size-sm".to_string() />
                    <Rating size={Size::Medium} value=4 name="size-md".to_string() />
                    <Rating size={Size::Large} value=5 name="size-lg".to_string() />
                </div>
            </section>
        </div>
    }
}
