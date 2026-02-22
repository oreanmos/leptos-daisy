use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarWidth {
    Narrow,
    #[default]
    Default,
    Wide,
}
impl SidebarWidth {
    fn cls(&self) -> &'static str {
        match self {
            Self::Narrow => "w-16",
            Self::Default => "w-64",
            Self::Wide => "w-80",
        }
    }
}

#[component]
pub fn SidebarShell(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex min-h-screen", &[], class);
    view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(optional)] width: SidebarWidth,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal(
        "flex flex-col shrink-0 bg-base-200 overflow-y-auto",
        &[width.cls()],
        class,
    );
    view! { <aside class=cls>{children()}</aside> }
}

#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex-1 overflow-y-auto", &[], class);
    view! { <main class=cls>{children()}</main> }
}
