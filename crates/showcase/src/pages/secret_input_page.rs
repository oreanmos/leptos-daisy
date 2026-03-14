use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SecretInputPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Secret Input"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A password input with a toggle button to show/hide the value."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r#"<SecretInput placeholder="Enter password" />"#
                >
                    <div class="max-w-sm">
                        <SecretInput placeholder="Enter password" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Color"
                    code=r#"<SecretInput color=Color::Primary placeholder="Primary" />"#
                >
                    <div class="max-w-sm space-y-2">
                        <SecretInput color=Color::Primary placeholder="Primary" />
                        <SecretInput color=Color::Error placeholder="Error" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<SecretInput size=Size::Small placeholder="Small" />
    <SecretInput size=Size::Large placeholder="Large" />"#
                >
                    <div class="max-w-sm space-y-2">
                        <SecretInput size=Size::Small placeholder="Small" />
                        <SecretInput placeholder="Default" />
                        <SecretInput size=Size::Large placeholder="Large" />
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
