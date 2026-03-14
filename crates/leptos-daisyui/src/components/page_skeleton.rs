//! PageSkeleton component — a loading skeleton layout.
//!
//! Renders a configurable skeleton with optional header, card grid, and row placeholders.

use crate::components::skeleton::Skeleton;
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

/// A page-level loading skeleton with configurable rows and optional card grid.
///
/// # Example
///
/// ```rust,ignore
/// view! {
///     <PageSkeleton rows=6 show_header=true show_cards=true cards=3 />
/// }
/// ```
#[component]
pub fn PageSkeleton(
    #[prop(optional, default = 4)] rows: usize,
    #[prop(optional, default = true)] show_header: bool,
    #[prop(optional, default = false)] show_cards: bool,
    #[prop(optional, default = 4)] cards: usize,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("space-y-6 animate-pulse", &[], class);

    view! {
        <div class=cls>
            {show_header.then(|| view! {
                <div class="space-y-3">
                    <Skeleton class="h-8 w-48" />
                    <Skeleton class="h-4 w-96" />
                </div>
            })}
            {show_cards.then(|| {
                let card_items: Vec<usize> = (0..cards).collect();
                view! {
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                        {card_items.into_iter().map(|_| view! {
                            <div class="card bg-base-200">
                                <div class="card-body gap-3">
                                    <Skeleton class="h-4 w-24" />
                                    <Skeleton class="h-8 w-16" />
                                </div>
                            </div>
                        }).collect_view()}
                    </div>
                }
            })}
            <div class="space-y-3">
                {(0..rows).map(|i| {
                    let width = match i % 3 {
                        0 => "w-full",
                        1 => "w-3/4",
                        _ => "w-5/6",
                    };
                    view! { <Skeleton class=format!("h-4 {width}") /> }
                }).collect_view()}
            </div>
        </div>
    }
    .add_any_attr(attrs)
}
