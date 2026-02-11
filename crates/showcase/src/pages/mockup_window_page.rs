use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupWindowPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mockup Window"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <MockupWindow class="border border-base-300">
                    <div class="flex justify-center px-4 py-16 bg-base-200">"Hello!"</div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Toolbar"</h2>
                <MockupWindow class="border border-base-300">
                    <div class="flex justify-center px-4 py-16 bg-base-200">"Content"</div>
                </MockupWindow>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Custom Background"</h2>
                <MockupWindow class="border border-base-300 bg-primary">
                    <div class="flex justify-center px-4 py-16 bg-primary text-primary-content">
                        "Primary themed window"
                    </div>
                </MockupWindow>
            </section>
        </div>
    }
}
