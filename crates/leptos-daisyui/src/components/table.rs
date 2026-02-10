//! Table component — daisyUI `table` + modifiers.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Table size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableSize {
    #[default]
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl TableSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::ExtraSmall => "table-xs",
            Self::Small => "table-sm",
            Self::Medium => "table-md",
            Self::Large => "table-lg",
            Self::ExtraLarge => "table-xl",
        }
    }
}

/// Table component.
#[component]
pub fn Table(
    children: Children,
    #[prop(optional)] size: TableSize,
    #[prop(optional)] zebra: bool,
    #[prop(optional)] pin_rows: bool,
    #[prop(optional)] pin_cols: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    let size_cls = size.cls();
    if !size_cls.is_empty() {
        mods.push(size_cls);
    }
    if zebra {
        mods.push("table-zebra");
    }
    if pin_rows {
        mods.push("table-pin-rows");
    }
    if pin_cols {
        mods.push("table-pin-cols");
    }
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "table",
        &mods,
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    view! { <table class={cls}>{children()}</table> }
}

/// Table header component.
#[component]
pub fn TableHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], if uc.is_empty() { None } else { Some(&uc) });
    view! { <thead class={cls}>{children()}</thead> }.into_any()
}

/// Table body component.
#[component]
pub fn TableBody(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], if uc.is_empty() { None } else { Some(&uc) });
    view! { <tbody class={cls}>{children()}</tbody> }.into_any()
}

/// Table row component.
#[component]
pub fn TableRow(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], if uc.is_empty() { None } else { Some(&uc) });
    view! { <tr class={cls}>{children()}</tr> }.into_any()
}

/// Table header cell component.
#[component]
pub fn TableHead(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], if uc.is_empty() { None } else { Some(&uc) });
    view! { <th class={cls}>{children()}</th> }.into_any()
}

/// Table data cell component.
#[component]
pub fn TableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], if uc.is_empty() { None } else { Some(&uc) });
    view! { <td class={cls}>{children.map(|c| c())}</td> }.into_any()
}

/// Table footer component.
#[component]
pub fn TableFooter(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class("", &[], if uc.is_empty() { None } else { Some(&uc) });
    view! { <tfoot class={cls}>{children()}</tfoot> }.into_any()
}
