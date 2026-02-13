//! Toast controller — programmatic toast notifications.

use leptos::prelude::*;

/// The color/severity of a toast message.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastColor {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl ToastColor {
    /// Returns the DaisyUI alert class for this color.
    pub fn alert_class(&self) -> &'static str {
        match self {
            Self::Default => "alert",
            Self::Info => "alert alert-info",
            Self::Success => "alert alert-success",
            Self::Warning => "alert alert-warning",
            Self::Error => "alert alert-error",
        }
    }
}

/// A single toast message.
#[derive(Clone, Debug)]
pub struct ToastMessage {
    /// Unique ID for this message.
    pub id: u64,
    /// The text content.
    pub content: String,
    /// The color/severity.
    pub color: ToastColor,
    /// Duration in milliseconds before auto-dismiss (0 = no auto-dismiss).
    pub duration_ms: u32,
}

/// A reactive controller for managing a stack of toast notifications.
///
/// # Example
///
/// ```rust,ignore
/// let toasts = use_toasts();
///
/// view! {
///     <button on:click=move |_| toasts.success("Saved!")>"Save"</button>
///     <button on:click=move |_| toasts.error("Something went wrong")>"Error"</button>
///     <ToastContainer controller=toasts />
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ToastController {
    messages: RwSignal<Vec<ToastMessage>>,
    next_id: RwSignal<u64>,
}

impl ToastController {
    /// Create a new toast controller.
    pub fn new() -> Self {
        Self {
            messages: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(0),
        }
    }

    /// Push a toast message with the given content, color, and duration.
    pub fn push(&self, content: impl Into<String>, color: ToastColor, duration_ms: u32) {
        let id = self.next_id.get_untracked();
        self.next_id.set(id + 1);
        let msg = ToastMessage {
            id,
            content: content.into(),
            color,
            duration_ms,
        };
        self.messages.update(|msgs| msgs.push(msg));

        if duration_ms > 0 {
            let messages = self.messages;
            set_timeout(
                move || {
                    messages.update(|msgs| msgs.retain(|m| m.id != id));
                },
                std::time::Duration::from_millis(duration_ms as u64),
            );
        }
    }

    /// Dismiss a specific toast by ID.
    pub fn dismiss(&self, id: u64) {
        self.messages.update(|msgs| msgs.retain(|m| m.id != id));
    }

    /// Push a success toast (auto-dismisses after 3 seconds).
    pub fn success(&self, content: impl Into<String>) {
        self.push(content, ToastColor::Success, 3000);
    }

    /// Push an error toast (auto-dismisses after 5 seconds).
    pub fn error(&self, content: impl Into<String>) {
        self.push(content, ToastColor::Error, 5000);
    }

    /// Push a warning toast (auto-dismisses after 4 seconds).
    pub fn warning(&self, content: impl Into<String>) {
        self.push(content, ToastColor::Warning, 4000);
    }

    /// Push an info toast (auto-dismisses after 3 seconds).
    pub fn info(&self, content: impl Into<String>) {
        self.push(content, ToastColor::Info, 3000);
    }

    /// Get the current list of messages as a read signal.
    pub fn messages(&self) -> RwSignal<Vec<ToastMessage>> {
        self.messages
    }
}

impl Default for ToastController {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new [`ToastController`].
pub fn use_toasts() -> ToastController {
    ToastController::new()
}

/// A component that renders the toast stack. Place this once in your app layout.
///
/// # Example
///
/// ```rust,ignore
/// let toasts = use_toasts();
/// view! {
///     <ToastContainer controller=toasts />
/// }
/// ```
#[component]
pub fn ToastContainer(controller: ToastController) -> impl IntoView {
    let messages = controller.messages();
    view! {
        <div class="toast toast-end toast-bottom z-50">
            {move || {
                messages.get().into_iter().map(|msg| {
                    let id = msg.id;
                    let class = msg.color.alert_class();
                    view! {
                        <div class=class>
                            <span>{msg.content}</span>
                            <button
                                class="btn btn-ghost btn-xs"
                                on:click=move |_| controller.dismiss(id)
                            >
                                "✕"
                            </button>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}
