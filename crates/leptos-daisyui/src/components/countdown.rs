//! Countdown component — daisyUI `countdown` with CSS variables.
use crate::utils::class::class_signal;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[component]
pub fn Countdown(
    #[prop(into)] value: i32,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("countdown", &[], class);
    let style = format!("--value:{};", value);
    view! {
        <span class=cls style=style role="timer" aria-live="polite"
            aria-label=move || aria_label.get()
        >{value}</span>
    }
    .add_any_attr(attrs)
}

#[component]
pub fn CountdownTimer(
    #[prop(optional, default = 0)] days: u32,
    #[prop(optional, default = 0)] hours: u32,
    #[prop(optional, default = 0)] minutes: u32,
    #[prop(optional, default = 0)] seconds: u32,
    #[prop(optional)] show_days: bool,
    #[prop(optional)] show_hours: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("", &[], class);
    view! {
        <div class=cls role="timer" aria-live="polite">
            {show_days.then(|| view! {
                <div class="flex flex-col">
                    <span class="countdown font-mono text-5xl" style=format!("--value:{}", days)>{days}</span>
                    <span class="text-sm">"days"</span>
                </div>
            })}
            {show_hours.then(|| view! {
                <div class="flex flex-col">
                    <span class="countdown font-mono text-5xl" style=format!("--value:{}", hours)>{hours}</span>
                    <span class="text-sm">"hours"</span>
                </div>
            })}
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl" style=format!("--value:{}", minutes)>{minutes}</span>
                <span class="text-sm">"min"</span>
            </div>
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl" style=format!("--value:{}", seconds)>{seconds}</span>
                <span class="text-sm">"sec"</span>
            </div>
        </div>
    }.add_any_attr(attrs)
}
