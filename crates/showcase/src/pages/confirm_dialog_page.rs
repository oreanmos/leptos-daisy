use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ConfirmDialogPage() -> impl IntoView {
    let modal = use_modal();
    let modal2 = use_modal();

    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Confirm Dialog"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A modal dialog for confirming destructive or important actions."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic (Error)"
                    code=r#"let modal = use_modal();
<Button color=Color::Error on_click=move |_| modal.open()>"Delete"</Button>
<ConfirmDialog
    controller=modal
    title="Delete Item"
    message="Are you sure? This cannot be undone."
    on_confirm=move |_| modal.close()
/>"#
                >
                    <Button color=Color::Error on_click=move |_| modal.open()>"Delete Item"</Button>
                    <ConfirmDialog
                        controller=modal
                        title="Delete Item"
                        message="Are you sure you want to delete this item? This action cannot be undone."
                        on_confirm=Callback::new(move |_| modal.close())
                    />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Custom Labels & Warning Color"
                    code=r#"<ConfirmDialog
    controller=modal
    title="Reset Settings"
    message="This will reset all settings to defaults."
    confirm_label="Reset"
    cancel_label="Keep"
    color=Color::Warning
    on_confirm=move |_| modal.close()
/>"#
                >
                    <Button color=Color::Warning on_click=move |_| modal2.open()>"Reset Settings"</Button>
                    <ConfirmDialog
                        controller=modal2
                        title="Reset Settings"
                        message="This will reset all settings to their default values. Are you sure?"
                        confirm_label="Reset".to_string()
                        cancel_label="Keep".to_string()
                        color=Color::Warning
                        on_confirm=Callback::new(move |_| modal2.close())
                    />
                </ComponentPreview>
            </section>
        </div>
    }
}
