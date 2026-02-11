use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SwapPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Swap"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Swap
                    on=Box::new(|| view! { <span>"ON"</span> }.into_any())
                    off=Box::new(|| view! { <span>"OFF"</span> }.into_any())
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Rotate Animation"</h2>
                <Swap
                    animation=SwapAnimation::Rotate
                    on=Box::new(|| view! { <span class="text-4xl">"🌞"</span> }.into_any())
                    off=Box::new(|| view! { <span class="text-4xl">"🌜"</span> }.into_any())
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Flip Animation"</h2>
                <Swap
                    animation=SwapAnimation::Flip
                    on=Box::new(|| view! { <span class="text-4xl">"😊"</span> }.into_any())
                    off=Box::new(|| view! { <span class="text-4xl">"😴"</span> }.into_any())
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Hamburger Menu"</h2>
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
            </section>
        </div>
    }
}
