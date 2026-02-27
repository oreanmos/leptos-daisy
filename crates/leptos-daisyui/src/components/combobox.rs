//! ComboBox component — searchable dropdown selector.

use crate::variants::color::Color;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::ev;
use leptos::prelude::*;

fn get_combobox_input_classes(color: Option<Color>, size: Option<Size>) -> Vec<String> {
    let mut m = Vec::new();
    if let Some(c) = color {
        let s = c.class("input");
        if !s.is_empty() {
            m.push(s);
        }
    }
    if let Some(s) = size {
        m.push(s.class("input"));
    }
    m
}

/// A searchable dropdown selector (combobox pattern).
///
/// Combines a text input for search with a dropdown panel of selectable items.
/// Uses `mousedown` on items (not blur timeout) for reliable selection.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <ComboBox
///         placeholder="Search users..."
///         on_search=move |ev: ev::Event| { /* filter items */ }
///         on_select=move |id: String| { /* handle selection */ }
///     >
///         <ComboBoxItem value="1">"Alice"</ComboBoxItem>
///         <ComboBoxItem value="2">"Bob"</ComboBoxItem>
///     </ComboBox>
/// }
/// ```
#[component]
pub fn ComboBox(
    #[prop(optional, into)] color: Option<Color>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] disabled: MaybeProp<bool>,
    #[prop(optional)] loading: bool,
    /// Called when the user types in the search input (raw event).
    on_search: Callback<ev::Event>,
    /// Called when the user selects an item (receives the item's `value`).
    on_select: Callback<String>,
    /// Called when the user clears the selection.
    #[prop(optional, into)]
    on_clear: Option<Callback<()>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
    children: Children,
) -> impl IntoView {
    let open = RwSignal::new(false);

    let input_mods = get_combobox_input_classes(color, size);
    let mut input_parts = vec!["input".to_string(), "input-bordered".to_string(), "w-full".to_string()];
    input_parts.extend(input_mods);
    let input_base: String = input_parts.join(" ");

    let handle_input = move |ev: ev::Event| {
        on_search.run(ev);
        open.set(true);
    };

    let handle_focus = move |_: ev::FocusEvent| {
        open.set(true);
    };

    let handle_blur = move |_: ev::FocusEvent| {
        open.set(false);
    };

    let handle_clear = move |ev: ev::MouseEvent| {
        ev.prevent_default();
        if let Some(cb) = on_clear {
            cb.run(());
        }
        open.set(false);
    };

    let input_cls = move || {
        match class.get() {
            Some(extra) => format!("{input_base} {extra}"),
            None => input_base.clone(),
        }
    };

    // Provide context for item selection
    provide_context(ComboBoxCtx { on_select, open });

    // Render children eagerly, control visibility with CSS
    let rendered_children = children();

    let dropdown_cls = move || {
        if open.get() {
            "absolute z-50 mt-1 w-full bg-base-100 border border-base-300 rounded-box shadow-lg max-h-60 overflow-y-auto"
        } else {
            "hidden"
        }
    };

    let clear_btn_cls = move || {
        if value.get().map(|v| !v.is_empty()).unwrap_or(false) {
            "btn btn-ghost btn-circle btn-xs absolute right-2"
        } else {
            "hidden"
        }
    };

    view! {
        <div class="relative">
            <div class="flex items-center">
                <input
                    type="text"
                    class=input_cls
                    placeholder=move || placeholder.get().unwrap_or_default()
                    disabled=move || disabled.get().unwrap_or(false)
                    prop:value=move || value.get().unwrap_or_default()
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                />
                <button
                    type="button"
                    class=clear_btn_cls
                    on:mousedown=handle_clear
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-3 h-3">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
            <div class=dropdown_cls>
                {loading.then(|| view! {
                    <div class="p-3 text-center text-base-content/60">
                        <span class="loading loading-spinner loading-sm"></span>
                    </div>
                })}
                <ul class="menu menu-compact p-1">
                    {rendered_children}
                </ul>
            </div>
        </div>
    }
    .add_any_attr(attrs)
}

/// Context for combobox item selection.
#[derive(Clone, Copy)]
struct ComboBoxCtx {
    on_select: Callback<String>,
    open: RwSignal<bool>,
}

/// A selectable item within a [`ComboBox`].
#[component]
pub fn ComboBoxItem(
    /// The value sent to `on_select` when this item is chosen.
    #[prop(into)]
    value: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<ComboBoxCtx>();
    let value_clone = value.clone();

    let handle_mousedown = move |ev: ev::MouseEvent| {
        ev.prevent_default(); // Prevent blur on the input
        if let Some(ctx) = ctx {
            ctx.on_select.run(value_clone.clone());
            ctx.open.set(false);
        }
    };

    let cls = move || class.get().unwrap_or_default();

    view! {
        <li>
            <a class=cls on:mousedown=handle_mousedown>
                {children()}
            </a>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combobox_input_classes_default() {
        let classes = get_combobox_input_classes(None, None);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_combobox_input_classes_color_size() {
        let classes = get_combobox_input_classes(Some(Color::Primary), Some(Size::Small));
        assert_eq!(classes, vec!["input-primary", "input-sm"]);
    }
}
