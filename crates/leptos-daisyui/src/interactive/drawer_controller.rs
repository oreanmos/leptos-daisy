//! Drawer controller — programmatic open/close for daisyUI drawers.

use leptos::prelude::*;

/// A reactive controller for managing drawer open/close state.
///
/// # Example
///
/// ```rust,ignore
/// let drawer = use_drawer();
///
/// view! {
///     <Drawer id="my-drawer" open=drawer.is_open() on_toggle=move |v| drawer.set(v)>
///         <DrawerContent>
///             <button on:click=move |_| drawer.open()>"Open Drawer"</button>
///         </DrawerContent>
///         <DrawerSide>
///             <DrawerOverlay drawer_id="my-drawer" />
///             <div class="menu bg-base-200 w-80 min-h-full p-4">
///                 <p>"Sidebar content"</p>
///             </div>
///         </DrawerSide>
///     </Drawer>
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct DrawerController {
    open: RwSignal<bool>,
}

impl DrawerController {
    /// Create a new drawer controller with closed state.
    pub fn new() -> Self {
        Self {
            open: RwSignal::new(false),
        }
    }

    /// Open the drawer.
    pub fn open(&self) {
        self.open.set(true);
    }

    /// Close the drawer.
    pub fn close(&self) {
        self.open.set(false);
    }

    /// Toggle the drawer state.
    pub fn toggle(&self) {
        self.open.update(|v| *v = !*v);
    }

    /// Set the drawer state directly (useful for `on_toggle` callbacks).
    pub fn set(&self, value: bool) {
        self.open.set(value);
    }

    /// Returns whether the drawer is currently open.
    pub fn is_open(&self) -> ReadSignal<bool> {
        self.open.read_only()
    }

    /// Get the underlying signal for binding to `open` props.
    pub fn signal(&self) -> RwSignal<bool> {
        self.open
    }
}

impl Default for DrawerController {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new [`DrawerController`].
pub fn use_drawer() -> DrawerController {
    DrawerController::new()
}
