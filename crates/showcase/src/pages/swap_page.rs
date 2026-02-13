use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SwapPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Swap"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Toggle between two states with animated transitions for icons, text, or custom content."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "swap", type_label: "base", description: "Base swap container" },
                    ClassEntry { name: "swap-on", type_label: "base", description: "Content shown when swap is active" },
                    ClassEntry { name: "swap-off", type_label: "base", description: "Content shown when swap is inactive" },
                    ClassEntry { name: "swap-indeterminate", type_label: "base", description: "Content shown in indeterminate state" },
                    ClassEntry { name: "swap-active", type_label: "state", description: "Activate the swap state" },
                    ClassEntry { name: "swap-rotate", type_label: "modifier", description: "Rotate animation on toggle" },
                    ClassEntry { name: "swap-flip", type_label: "modifier", description: "Flip animation on toggle" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r#"<Swap
    on=Box::new(|| view! { <span>"ON"</span> }.into_any())
    off=Box::new(|| view! { <span>"OFF"</span> }.into_any())
/>"#
                >
                    <Swap
                        on=Box::new(|| view! { <span>"ON"</span> }.into_any())
                        off=Box::new(|| view! { <span>"OFF"</span> }.into_any())
                    />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Rotate Animation"
                    code=r#"<Swap
    animation=SwapAnimation::Rotate
    on=Box::new(|| view! { <span class="text-4xl">"🌞"</span> }.into_any())
    off=Box::new(|| view! { <span class="text-4xl">"🌜"</span> }.into_any())
/>"#
                >
                    <Swap
                        animation=SwapAnimation::Rotate
                        on=Box::new(|| view! { <span class="text-4xl">"🌞"</span> }.into_any())
                        off=Box::new(|| view! { <span class="text-4xl">"🌜"</span> }.into_any())
                    />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Flip Animation"
                    code=r#"<Swap
    animation=SwapAnimation::Flip
    on=Box::new(|| view! { <span class="text-4xl">"😊"</span> }.into_any())
    off=Box::new(|| view! { <span class="text-4xl">"😴"</span> }.into_any())
/>"#
                >
                    <Swap
                        animation=SwapAnimation::Flip
                        on=Box::new(|| view! { <span class="text-4xl">"😊"</span> }.into_any())
                        off=Box::new(|| view! { <span class="text-4xl">"😴"</span> }.into_any())
                    />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Hamburger Menu"
                    code=r#"<Swap
    animation=SwapAnimation::Rotate
    on=Box::new(|| view! {
        <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 512 512">
            <polygon points="400 145.49 366.51 112 256 222.51 145.49 112 112 145.49 222.51 256 112 366.51 145.49 400 256 289.49 366.51 400 400 366.51 289.49 256 400 145.49" />
        </svg>
    }.into_any())
    off=Box::new(|| view! {
        <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 512 512">
            <path d="M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" />
        </svg>
    }.into_any())
/>"#
                >
                    <Swap
                        animation=SwapAnimation::Rotate
                        on=Box::new(|| view! {
                            <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 512 512">
                                <polygon points="400 145.49 366.51 112 256 222.51 145.49 112 112 145.49 222.51 256 112 366.51 145.49 400 256 289.49 366.51 400 400 366.51 289.49 256 400 145.49" />
                            </svg>
                        }.into_any())
                        off=Box::new(|| view! {
                            <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 512 512">
                                <path d="M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" />
                            </svg>
                        }.into_any())
                    />
                </ComponentPreview>
            </section>
        </div>
    }
}
