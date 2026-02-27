//! ConfirmDialog component — a modal dialog for confirming destructive actions.

use crate::interactive::modal_controller::ModalController;
use crate::variants::color::Color;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

/// A confirmation dialog built on `ControlledModal`.
///
/// Shows a title, message, and Cancel/Confirm buttons. The confirm button
/// uses the specified color (defaults to `Color::Error` for destructive actions).
///
/// # Example
///
/// ```rust,ignore
/// let modal = use_modal();
///
/// view! {
///     <Button on_click=move |_| modal.open()>"Delete"</Button>
///     <ConfirmDialog
///         controller=modal
///         title="Delete Item"
///         message="Are you sure? This cannot be undone."
///         on_confirm=move |_| { /* delete logic */ }
///     />
/// }
/// ```
#[component]
pub fn ConfirmDialog(
    controller: ModalController,
    #[prop(into)] title: String,
    #[prop(into)] message: String,
    #[prop(optional, into)] confirm_label: Option<String>,
    #[prop(optional, into)] cancel_label: Option<String>,
    #[prop(optional)] color: Option<Color>,
    /// Optional loading signal — when true, shows "Processing..." and disables confirm.
    #[prop(optional)]
    loading: Option<RwSignal<bool>>,
    on_confirm: Callback<()>,
    #[prop(optional, into)] on_cancel: Option<Callback<()>>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let confirm_text = confirm_label.unwrap_or_else(|| "Confirm".to_string());
    let cancel_text = cancel_label.unwrap_or_else(|| "Cancel".to_string());
    let btn_color = color.unwrap_or(Color::Error);

    let is_loading = Signal::derive(move || loading.map(|l| l.get()).unwrap_or(false));

    let confirm_text_clone = confirm_text.clone();
    let handle_confirm = move |_: ev::MouseEvent| {
        on_confirm.run(());
    };

    let handle_cancel = move |_: ev::MouseEvent| {
        if let Some(cb) = on_cancel {
            cb.run(());
        }
        controller.close();
    };

    // Build color class for confirm button
    let color_cls = {
        let s = btn_color.class("btn");
        if s.is_empty() {
            "btn".to_string()
        } else {
            format!("btn {s}")
        }
    };

    view! {
        <crate::interactive::modal_controller::ControlledModal
            controller=controller
            close_on_backdrop=true
            close_on_escape=true
        >
            <crate::components::modal::ModalBox>
                <h3 class="font-bold text-lg">{title}</h3>
                <p class="py-4">{message}</p>
                <crate::components::modal::ModalAction>
                    <button type="button" class="btn btn-ghost" on:click=handle_cancel>
                        {cancel_text}
                    </button>
                    <button
                        type="button"
                        class=color_cls
                        disabled=is_loading
                        on:click=handle_confirm
                    >
                        {move || {
                            if is_loading.get() {
                                "Processing\u{2026}".to_string()
                            } else {
                                confirm_text_clone.clone()
                            }
                        }}
                    </button>
                </crate::components::modal::ModalAction>
            </crate::components::modal::ModalBox>
        </crate::interactive::modal_controller::ControlledModal>
    }
    .add_any_attr(attrs)
}
