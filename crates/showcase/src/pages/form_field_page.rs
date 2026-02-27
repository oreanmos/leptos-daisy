use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FormFieldPage() -> impl IntoView {
    let error_signal = RwSignal::new(Some("This field is required".to_string()));

    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Form Field"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A structured form field wrapper with label, hint, error, and required indicator."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r#"<FormField label="Username">
    <Input placeholder="Enter username" />
</FormField>"#
                >
                    <div class="max-w-sm">
                        <FormField label="Username">
                            <Input placeholder="Enter username" />
                        </FormField>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Hint"
                    code=r#"<FormField label="Email" hint="We'll never share your email.">
    <Input placeholder="you@example.com" />
</FormField>"#
                >
                    <div class="max-w-sm">
                        <FormField label="Email" hint="We'll never share your email.">
                            <Input placeholder="you@example.com" />
                        </FormField>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Required with Error"
                    code=r#"<FormField label="Password" required=true error="This field is required">
    <Input placeholder="Enter password" />
</FormField>"#
                >
                    <div class="max-w-sm">
                        <FormField label="Password" required=true error=Signal::derive(move || error_signal.get())>
                            <Input placeholder="Enter password" />
                        </FormField>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
