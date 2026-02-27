use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ComboboxPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"ComboBox"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A searchable dropdown selector. Combines a text input with a dropdown panel."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r#"<ComboBox
    placeholder="Search..."
    on_search=move |_| {}
    on_select=move |id| log!("Selected: {id}")
>
    <ComboBoxItem value="1">"Alice"</ComboBoxItem>
    <ComboBoxItem value="2">"Bob"</ComboBoxItem>
</ComboBox>"#
                >
                    <div class="max-w-sm">
                        <ComboBox
                            placeholder="Search users..."
                            on_search=Callback::new(move |_: leptos::ev::Event| {})
                            on_select=Callback::new(move |_: String| {})
                        >
                            <ComboBoxItem value="1">"Alice Johnson"</ComboBoxItem>
                            <ComboBoxItem value="2">"Bob Smith"</ComboBoxItem>
                            <ComboBoxItem value="3">"Carol Williams"</ComboBoxItem>
                            <ComboBoxItem value="4">"David Brown"</ComboBoxItem>
                        </ComboBox>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
