use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupWindowPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mockup Window"</h1>

            <section>
                <ComponentPreview
                    title="Basic"
                    code=r#"<MockupWindow class="border border-base-300">
    <div class="flex justify-center px-4 py-16 bg-base-200">"Hello!"</div>
</MockupWindow>"#
                >
                    <MockupWindow class="border border-base-300">
                        <div class="flex justify-center px-4 py-16 bg-base-200">"Hello!"</div>
                    </MockupWindow>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Toolbar"
                    code=r#"<MockupWindow class="border border-base-300">
    <div class="flex justify-center px-4 py-16 bg-base-200">"Content"</div>
</MockupWindow>"#
                >
                    <MockupWindow class="border border-base-300">
                        <div class="flex justify-center px-4 py-16 bg-base-200">"Content"</div>
                    </MockupWindow>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Custom Background"
                    code=r#"<MockupWindow class="border border-base-300 bg-primary">
    <div class="flex justify-center px-4 py-16 bg-primary text-primary-content">
        "Primary themed window"
    </div>
</MockupWindow>"#
                >
                    <MockupWindow class="border border-base-300 bg-primary">
                        <div class="flex justify-center px-4 py-16 bg-primary text-primary-content">
                            "Primary themed window"
                        </div>
                    </MockupWindow>
                </ComponentPreview>
            </section>
        </div>
    }
}
