//! Modal controller — programmatic open/close for daisyUI modals.

use leptos::prelude::*;

/// A reactive controller for managing modal open/close state.
///
/// # Example
///
/// ```rust,ignore
/// let modal = use_modal();
///
/// view! {
///     <button on:click=move |_| modal.open()>"Open"</button>
///     <Modal open=modal.is_open()>
///         <ModalBox>
///             <p>"Hello!"</p>
///             <ModalAction>
///                 <button class="btn" on:click=move |_| modal.close()>"Close"</button>
///             </ModalAction>
///         </ModalBox>
///         <ModalBackdrop />
///     </Modal>
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ModalController {
    open: RwSignal<bool>,
}

impl ModalController {
    /// Create a new modal controller with closed state.
    pub fn new() -> Self {
        Self {
            open: RwSignal::new(false),
        }
    }

    /// Open the modal.
    pub fn open(&self) {
        self.open.set(true);
    }

    /// Close the modal.
    pub fn close(&self) {
        self.open.set(false);
    }

    /// Toggle the modal state.
    pub fn toggle(&self) {
        self.open.update(|v| *v = !*v);
    }

    /// Returns whether the modal is currently open.
    pub fn is_open(&self) -> ReadSignal<bool> {
        self.open.read_only()
    }

    /// Get the underlying signal for binding to `open` props.
    pub fn signal(&self) -> RwSignal<bool> {
        self.open
    }
}

impl Default for ModalController {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new [`ModalController`].
pub fn use_modal() -> ModalController {
    ModalController::new()
}
