use crate::components::breadcrumbs::{BreadcrumbItem, Breadcrumbs};
use crate::utils::class::class_signal;
use leptos::prelude::*;

/// Page header with optional structured title, subtitle, breadcrumbs, and actions.
///
/// When `title` is provided, renders a structured layout with optional breadcrumbs,
/// title, subtitle, and actions (via `children`). When `title` is `None`, renders
/// only `children` for full backward compatibility.
#[component]
pub fn PageHeader(
    #[prop(optional, into)] title: MaybeProp<String>,
    #[prop(optional, into)] subtitle: MaybeProp<String>,
    #[prop(optional)] breadcrumbs: Option<Vec<BreadcrumbItem>>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal(
        "border-b border-base-300 bg-base-100 px-4 py-4 sm:px-6 lg:px-8",
        &[],
        class,
    );

    let has_title = move || title.get().is_some();

    let title_view = move || {
        title.get().map(|t| {
            let bc = breadcrumbs.clone();
            let sub = subtitle.get();
            view! {
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                    <div>
                        {bc.map(|items| view! { <div class="mb-2"><Breadcrumbs items=items /></div> })}
                        <h1 class="text-2xl font-bold leading-7">{t}</h1>
                        {sub.map(|s| view! { <p class="mt-1 text-sm text-base-content/70">{s}</p> })}
                    </div>
                </div>
            }
        })
    };

    view! {
        <header class=cls>
            {title_view}
            {children.map(|c| {
                if has_title() {
                    view! { <div class="flex items-center gap-2 mt-4 sm:mt-0">{c()}</div> }.into_any()
                } else {
                    c().into_any()
                }
            })}
        </header>
    }
}

#[component]
pub fn PageHeaderTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("text-2xl font-bold leading-7", &[], class);
    view! { <h1 class=cls>{children()}</h1> }
}

#[component]
pub fn PageHeaderSubtitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("mt-1 text-sm text-base-content/70", &[], class);
    view! { <p class=cls>{children()}</p> }
}

#[component]
pub fn PageHeaderActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("flex items-center gap-2", &[], class);
    view! { <div class=cls>{children()}</div> }
}
