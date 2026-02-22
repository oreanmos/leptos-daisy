//! Dock component — daisyUI `dock`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct DockItem {
    pub label: String,
    pub icon: Arc<dyn Fn() -> AnyView + Send + Sync>,
    pub active: bool,
    pub href: Option<String>,
}

impl fmt::Debug for DockItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DockItem")
            .field("label", &self.label)
            .field("active", &self.active)
            .field("href", &self.href)
            .finish_non_exhaustive()
    }
}

impl DockItem {
    pub fn new<V>(label: impl Into<String>, icon: V) -> Self
    where
        V: IntoView + Clone + Send + Sync + 'static,
    {
        let icon = icon.clone();
        Self {
            label: label.into(),
            icon: Arc::new(move || icon.clone().into_any()),
            active: false,
            href: None,
        }
    }
    pub fn active<V>(label: impl Into<String>, icon: V) -> Self
    where
        V: IntoView + Clone + Send + Sync + 'static,
    {
        let icon = icon.clone();
        Self {
            label: label.into(),
            icon: Arc::new(move || icon.clone().into_any()),
            active: true,
            href: None,
        }
    }
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[component]
pub fn Dock(
    #[prop(into)] items: Vec<DockItem>,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] on_item_click: Option<Callback<usize>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut mods = Vec::new();
    if let Some(s) = size {
        mods.push(s.class("dock"));
    }
    let refs: Vec<&str> = mods.iter().map(|s| s.as_str()).collect();
    let cls = class_signal("dock", &refs, class);
    let handle_click = move |index: usize| {
        if let Some(cb) = on_item_click {
            cb.run(index);
        }
    };
    view! {
        <div class=cls>
            {items.into_iter().enumerate().map(|(index, item)| {
                let item_class = if item.active { "dock-active" } else { "" };
                view! {
                    {if let Some(href) = item.href {
                        view! {
                            <a href=href class=item_class aria-current={if item.active { Some("page") } else { None }}>
                                <span class="dock-icon">{(item.icon)()}</span>
                                <span class="dock-label">{item.label}</span>
                            </a>
                        }.into_any()
                    } else {
                        view! {
                            <button class=item_class on:click=move |_| handle_click(index) type="button"
                                aria-current={if item.active { Some("page") } else { None }}>
                                <span class="dock-icon">{(item.icon)()}</span>
                                <span class="dock-label">{item.label}</span>
                            </button>
                        }.into_any()
                    }}
                }
            }).collect::<Vec<_>>()}
        </div>
    }.add_any_attr(attrs)
}

#[component]
pub fn DockItemComponent(
    children: Children,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let mut mods: Vec<&str> = Vec::new();
    if active {
        mods.push("dock-active");
    }
    let cls = class_signal("", &mods, class);
    let handle_click = move |_| {
        if let Some(cb) = on_click {
            cb.run(());
        }
    };
    if let Some(href) = href {
        view! { <a class=cls href=href aria-current={if active { Some("page") } else { None }}>{children()}</a> }
            .add_any_attr(attrs)
            .into_any()
    } else {
        view! { <button class=cls on:click=handle_click type="button" aria-current={if active { Some("page") } else { None }}>{children()}</button> }
            .add_any_attr(attrs)
            .into_any()
    }
}
