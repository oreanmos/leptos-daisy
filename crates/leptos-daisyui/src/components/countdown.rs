//! Countdown component — daisyUI `countdown` with CSS variables.
use crate::utils::class::build_class;
use leptos::prelude::*;

/// Countdown component for timer display.
/// Uses CSS `--value` variable to set the number.
#[component]
pub fn Countdown(
    /// The value to display (0-99 typically)
    #[prop(into)]
    value: i32,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let cls = build_class(
        "countdown",
        &[],
        if uc.is_empty() {
            None
        } else {
            Some(uc.as_str())
        },
    );
    let style = format!("--value:{};", value);
    view! { <span class={cls} style={style}>{value}</span> }
}

/// Countdown with multiple units (days, hours, minutes, seconds).
#[component]
pub fn CountdownTimer(
    /// Days value
    #[prop(optional, default = 0)]
    days: u32,
    /// Hours value
    #[prop(optional, default = 0)]
    hours: u32,
    /// Minutes value
    #[prop(optional, default = 0)]
    minutes: u32,
    /// Seconds value
    #[prop(optional, default = 0)]
    seconds: u32,
    /// Show days
    #[prop(optional)]
    show_days: bool,
    /// Show hours
    #[prop(optional)]
    show_hours: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let uc = class.get().unwrap_or_default();
    let base_cls = "";
    let final_cls = if uc.is_empty() {
        base_cls.to_string()
    } else {
        uc
    };

    view! {
        <div class={final_cls}>
            {show_days.then(|| view! {
                <div class="flex flex-col">
                    <span class="countdown font-mono text-5xl" style={format!("--value:{}", days)}>
                        {days}
                    </span>
                    <span class="text-sm">"days"</span>
                </div>
            })}
            {show_hours.then(|| view! {
                <div class="flex flex-col">
                    <span class="countdown font-mono text-5xl" style={format!("--value:{}", hours)}>
                        {hours}
                    </span>
                    <span class="text-sm">"hours"</span>
                </div>
            })}
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl" style={format!("--value:{}", minutes)}>
                    {minutes}
                </span>
                <span class="text-sm">"min"</span>
            </div>
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl" style={format!("--value:{}", seconds)}>
                    {seconds}
                </span>
                <span class="text-sm">"sec"</span>
            </div>
        </div>
    }
}
