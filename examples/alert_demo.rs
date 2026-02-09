//! Alert Component Demo
//!
//! This example demonstrates all the Alert component variants and styles.

use leptos::*;
use leptos_daisy::components::alert::{AlertDirection, AlertStyle, AlertVariant};
use leptos_daisy::components::*;

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <div class="space-y-4 p-4">
            <h1 class="text-2xl font-bold mb-6">"Alert Component Demo"</h1>

            // Basic variants
            <h2 class="text-xl font-semibold mt-8 mb-4">"Basic Variants"</h2>
            <div class="space-y-2">
                <Alert variant=AlertVariant::Info>
                    <span>"Info: This is an informational message."</span>
                </Alert>
                <Alert variant=AlertVariant::Success>
                    <span>"Success: Operation completed successfully!"</span>
                </Alert>
                <Alert variant=AlertVariant::Warning>
                    <span>"Warning: Please check your input."</span>
                </Alert>
                <Alert variant=AlertVariant::Error>
                    <span>"Error: Something went wrong!"</span>
                </Alert>
            </div>

            // Alert styles
            <h2 class="text-xl font-semibold mt-8 mb-4">"Alert Styles (Info variant)"</h2>
            <div class="space-y-2">
                <Alert variant=AlertVariant::Info style=AlertStyle::Default>
                    <span>"Default style"</span>
                </Alert>
                <Alert variant=AlertVariant::Info style=AlertStyle::Outline>
                    <span>"Outline style"</span>
                </Alert>
                <Alert variant=AlertVariant::Info style=AlertStyle::Dash>
                    <span>"Dash style"</span>
                </Alert>
                <Alert variant=AlertVariant::Info style=AlertStyle::Soft>
                    <span>"Soft style"</span>
                </Alert>
            </div>

            // Alert directions
            <h2 class="text-xl font-semibold mt-8 mb-4">"Alert Directions"</h2>
            <div class="space-y-2">
                <Alert variant=AlertVariant::Info direction=AlertDirection::Default>
                    <span>"Default direction"</span>
                </Alert>
                <Alert variant=AlertVariant::Info direction=AlertDirection::Vertical>
                    <span>"Vertical direction"</span>
                </Alert>
                <Alert variant=AlertVariant::Info direction=AlertDirection::Horizontal>
                    <span>"Horizontal direction"</span>
                </Alert>
            </div>

            // Combined examples
            <h2 class="text-xl font-semibold mt-8 mb-4">"Combined Examples"</h2>
            <div class="space-y-2">
                <Alert variant=AlertVariant::Success style=AlertStyle::Outline icon=true>
                    <span>"Success outline with icon"</span>
                </Alert>
                <Alert variant=AlertVariant::Warning style=AlertStyle::Soft direction=AlertDirection::Vertical>
                    <span>"Warning soft vertical"</span>
                </Alert>
                <Alert variant=AlertVariant::Error style=AlertStyle::Dash icon=true>
                    <span>"Error dash with icon"</span>
                </Alert>
            </div>

            // SimpleAlert examples
            <h2 class="text-xl font-semibold mt-8 mb-4">"SimpleAlert Examples"</h2>
            <div class="space-y-2">
                <SimpleAlert
                    text="Simple info alert".to_string()
                    variant=AlertVariant::Info
                    style=AlertStyle::Outline
                />
                <SimpleAlert
                    text="Simple success alert".to_string()
                    variant=AlertVariant::Success
                    style=AlertStyle::Soft
                />
            </div>
        </div>
    }
}
