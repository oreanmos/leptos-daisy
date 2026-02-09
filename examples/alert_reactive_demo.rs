//! Reactive Alert Component Demo
//!
//! This example demonstrates the ReactiveAlert component with dynamic signal-based updates.

use leptos::*;
use leptos_daisy::components::alert::{AlertDirection, AlertStyle, AlertVariant};
use leptos_daisy::components::*;

#[component]
pub fn ReactiveAlertDemo() -> impl IntoView {
    // Create reactive signals for alert properties
    let alert_variant = create_signal(AlertVariant::Info);
    let alert_style = create_signal(AlertStyle::Default);
    let alert_direction = create_signal(AlertDirection::Default);
    let show_icon = create_signal(false);
    let message = create_signal("Initial message".to_string());

    // Derived signal for alert count
    let alert_count = create_signal(0);

    // Effect to update message when count changes
    create_effect(move |_| {
        let count = alert_count.get();
        if count > 0 {
            message.set(format!("Alert triggered {} times!", count));
        }
    });

    view! {
        <div class="space-y-4 p-4">
            <h1 class="text-2xl font-bold mb-6">"Reactive Alert Demo"</h1>

            <div class="flex space-x-2 mb-4">
                <button
                    class="btn btn-primary"
                    on:click=move |_| {
                        alert_variant.set(AlertVariant::Info);
                        alert_style.set(AlertStyle::Default);
                        alert_direction.set(AlertDirection::Default);
                        show_icon.set(false);
                        alert_count.update(|c| *c += 1);
                    }
                >
                    "Info Alert"
                </button>

                <button
                    class="btn btn-success"
                    on:click=move |_| {
                        alert_variant.set(AlertVariant::Success);
                        alert_style.set(AlertStyle::Outline);
                        alert_direction.set(AlertDirection::Default);
                        show_icon.set(true);
                        alert_count.update(|c| *c += 1);
                    }
                >
                    "Success Outline"
                </button>

                <button
                    class="btn btn-warning"
                    on:click=move |_| {
                        alert_variant.set(AlertVariant::Warning);
                        alert_style.set(AlertStyle::Soft);
                        alert_direction.set(AlertDirection::Vertical);
                        show_icon.set(false);
                        alert_count.update(|c| *c += 1);
                    }
                >
                    "Warning Soft Vertical"
                </button>

                <button
                    class="btn btn-error"
                    on:click=move |_| {
                        alert_variant.set(AlertVariant::Error);
                        alert_style.set(AlertStyle::Dash);
                        alert_direction.set(AlertDirection::Horizontal);
                        show_icon.set(true);
                        alert_count.update(|c| *c += 1);
                    }
                >
                    "Error Dash Horizontal"
                </button>
            </div>

            // Reactive Alert that responds to all signal changes
            <ReactiveAlert
                variant=alert_variant
                style=alert_style
                direction=alert_direction
                icon=show_icon.get()
            >
                {move || message.get()}
            </ReactiveAlert>

            <div class="mt-4 p-4 bg-base-200 rounded-lg">
                <h3 class="font-semibold mb-2">"Current Alert State:"</h3>
                <p>"Variant: " {move || format!("{:?}", alert_variant.get())}</p>
                <p>"Style: " {move || format!("{:?}", alert_style.get())}</p>
                <p>"Direction: " {move || format!("{:?}", alert_direction.get())}</p>
                <p>"Icon: " {move || if show_icon.get() { "Yes" } else { "No" }}</p>
                <p>"Count: " {move || alert_count.get().to_string()}</p>
            </div>

            // Conditional alert example
            <h2 class="text-xl font-semibold mt-8 mb-4">"Conditional Alert Example"</h2>
            <div class="space-y-2">
                {move || {
                    if alert_count.get() > 3 {
                        Some(view! {
                            <Alert variant=AlertVariant::Success style=AlertStyle::Soft>
                                "You've triggered more than 3 alerts! Great job!"
                            </Alert>
                        })
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}
