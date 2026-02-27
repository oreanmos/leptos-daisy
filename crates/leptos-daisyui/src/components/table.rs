//! Table component and subcomponents — daisyUI `table`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

fn get_table_classes(
    zebra: bool,
    pin_rows: bool,
    pin_cols: bool,
    size: Option<Size>,
) -> Vec<String> {
    let mut m = Vec::new();
    if zebra {
        m.push("table-zebra".into());
    }
    if pin_rows {
        m.push("table-pin-rows".into());
    }
    if pin_cols {
        m.push("table-pin-cols".into());
    }
    if let Some(s) = size {
        m.push(s.class("table"));
    }
    m
}

/// A daisyUI table.
#[component]
pub fn Table(
    children: Children,
    #[prop(optional)] zebra: bool,
    #[prop(optional)] pin_rows: bool,
    #[prop(optional)] pin_cols: bool,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mods = get_table_classes(zebra, pin_rows, pin_cols, size);
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("table", &refs, class);
    view! { <table class=cls>{children()}</table> }.add_any_attr(attrs)
}

/// Table head (`<thead>`).
#[component]
pub fn TableHead(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = move || class.get().unwrap_or_default();
    view! { <thead class=cls>{children()}</thead> }.add_any_attr(attrs)
}

/// Table body (`<tbody>`).
#[component]
pub fn TableBody(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = move || class.get().unwrap_or_default();
    view! { <tbody class=cls>{children()}</tbody> }.add_any_attr(attrs)
}

/// Table footer (`<tfoot>`).
#[component]
pub fn TableFoot(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = move || class.get().unwrap_or_default();
    view! { <tfoot class=cls>{children()}</tfoot> }.add_any_attr(attrs)
}

fn get_row_classes(hover: bool, active: bool) -> Vec<&'static str> {
    let mut m = Vec::new();
    if hover {
        m.push("hover");
    }
    if active {
        m.push("active");
    }
    m
}

/// Table row (`<tr>`) with optional `hover` and `active` modifiers.
#[component]
pub fn TableRow(
    children: Children,
    #[prop(optional)] hover: bool,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mods = get_row_classes(hover, active);
    let cls = class_signal("", &mods, class);
    view! { <tr class=cls>{children()}</tr> }.add_any_attr(attrs)
}

/// Table header cell (`<th>`).
#[component]
pub fn TableHeaderCell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = move || class.get().unwrap_or_default();
    view! { <th class=cls>{children()}</th> }.add_any_attr(attrs)
}

/// Table data cell (`<td>`).
#[component]
pub fn TableCell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = move || class.get().unwrap_or_default();
    view! { <td class=cls>{children()}</td> }.add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_classes_default() {
        let classes = get_table_classes(false, false, false, None);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_table_classes_zebra() {
        let classes = get_table_classes(true, false, false, None);
        assert_eq!(classes, vec!["table-zebra"]);
    }

    #[test]
    fn test_table_classes_pins() {
        let classes = get_table_classes(false, true, true, None);
        assert_eq!(classes, vec!["table-pin-rows", "table-pin-cols"]);
    }

    #[test]
    fn test_table_classes_size() {
        let classes = get_table_classes(false, false, false, Some(Size::Small));
        assert_eq!(classes, vec!["table-sm"]);
    }

    #[test]
    fn test_row_classes_default() {
        let classes = get_row_classes(false, false);
        assert!(classes.is_empty());
    }

    #[test]
    fn test_row_classes_hover_active() {
        let classes = get_row_classes(true, true);
        assert_eq!(classes, vec!["hover", "active"]);
    }
}
