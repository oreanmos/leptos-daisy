//! Dock component — daisyUI `dock`.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct DockItem {
    pub label: String,
    pub icon: String,
    pub active: bool,
    pub href: Option<String>,
}
impl DockItem {
    pub fn new(label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            active: false,
            href: None,
        }
    }
    pub fn active(label: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
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
                                <span class="dock-icon" inner_html={item.icon.clone()}></span>
                                <span class="dock-label">{item.label.clone()}</span>
                            </a>
                        }.into_any()
                    } else {
                        view! {
                            <button class=item_class on:click=move |_| handle_click(index) type="button"
                                aria-current={if item.active { Some("page") } else { None }}>
                                <span class="dock-icon" inner_html={item.icon.clone()}></span>
                                <span class="dock-label">{item.label.clone()}</span>
                            </button>
                        }.into_any()
                    }}
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn DockItemComponent(
    children: Children,
    #[prop(optional)] active: bool,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional, into)] class: MaybeProp<String>,
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
    view! {
        {if let Some(href) = href {
            view! { <a class=cls href=href aria-current={if active { Some("page") } else { None }}>{children()}</a> }.into_any()
        } else {
            view! { <button class=cls on:click=handle_click type="button" aria-current={if active { Some("page") } else { None }}>{children()}</button> }.into_any()
        }}
    }
}
