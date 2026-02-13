use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AlertPage() -> impl IntoView {
    // State for reactive alert example
    let (alert_color, set_alert_color) = signal("info");
    let (show_alert, set_show_alert) = signal(true);

    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Alert"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Alerts inform users about important events."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Variants"
                    code=r#"<Alert color={Color::Info}>"This is an info alert."</Alert>
<Alert color={Color::Success}>"This is a success alert."</Alert>
<Alert color={Color::Warning}>"This is a warning alert."</Alert>
<Alert color={Color::Error}>"This is an error alert."</Alert>"#
                >
                    <div class="space-y-2 w-full">
                        <Alert color={Color::Info}>"This is an info alert."</Alert>
                        <Alert color={Color::Success}>"This is a success alert."</Alert>
                        <Alert color={Color::Warning}>"This is a warning alert."</Alert>
                        <Alert color={Color::Error}>"This is an error alert."</Alert>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Styles"
                    code=r#"<Alert color={Color::Info} style={AlertStyle::Outline}>
    "Outline info alert"
</Alert>
<Alert color={Color::Success} style={AlertStyle::Dash}>
    "Dash success alert"
</Alert>
<Alert color={Color::Warning} style={AlertStyle::Soft}>
    "Soft warning alert"
</Alert>
<Alert color={Color::Error} style={AlertStyle::Soft}>
    "Soft error alert"
</Alert>"#
                >
                    <div class="space-y-2 w-full">
                        <Alert color={Color::Info} style={AlertStyle::Outline}>"Outline info alert"</Alert>
                        <Alert color={Color::Success} style={AlertStyle::Dash}>"Dash success alert"</Alert>
                        <Alert color={Color::Warning} style={AlertStyle::Soft}>"Soft warning alert"</Alert>
                        <Alert color={Color::Error} style={AlertStyle::Soft}>"Soft error alert"</Alert>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Layout Directions"
                     code=r#"<Alert color={Color::Info} direction={AlertDirection::Vertical}>
    <AlertIcon>"ℹ"</AlertIcon>
    <AlertTitle>"Info"</AlertTitle>
    <AlertContent>
        "This alert uses vertical layout for better readability on mobile devices."
    </AlertContent>
</Alert>

<Alert color={Color::Success} direction={AlertDirection::Horizontal}>
    <AlertIcon>"✓"</AlertIcon>
    <AlertTitle>"Success"</AlertTitle>
    <AlertContent>"This alert uses horizontal layout for compact display."</AlertContent>
</Alert>"#
                >
                    <div class="space-y-4 w-full">
                        <div>
                            <h3 class="text-lg font-medium mb-2">"Vertical Layout"</h3>
                            <Alert color={Color::Info} direction={AlertDirection::Vertical}>
                                <AlertIcon>"ℹ"</AlertIcon>
                                <AlertTitle>"Info"</AlertTitle>
                                <AlertContent>"This alert uses vertical layout for better readability on mobile devices."</AlertContent>
                            </Alert>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium mb-2">"Horizontal Layout"</h3>
                            <Alert color={Color::Success} direction={AlertDirection::Horizontal}>
                                <AlertIcon>"✓"</AlertIcon>
                                <AlertTitle>"Success"</AlertTitle>
                                <AlertContent>"This alert uses horizontal layout for compact display."</AlertContent>
                            </Alert>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Sub-components"
                     code=r#"<Alert color={Color::Info}>
    <AlertIcon>"ℹ"</AlertIcon>
    <AlertTitle>"Info"</AlertTitle>
    <AlertContent>"New software update available."</AlertContent>
    <AlertActions>
        <button class="btn btn-sm">"Deny"</button>
        <button class="btn btn-sm btn-primary">"Accept"</button>
    </AlertActions>
</Alert>"#
                >
                     <Alert color={Color::Info}>
                        <AlertIcon>"ℹ"</AlertIcon>
                        <AlertTitle>"Info"</AlertTitle>
                        <AlertContent>"New software update available."</AlertContent>
                        <AlertActions>
                            <button class="btn btn-sm">"Deny"</button>
                            <button class="btn btn-sm btn-primary">"Accept"</button>
                        </AlertActions>
                    </Alert>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Dismissible Alert"
                     code=r#"let (show_alert, set_show_alert) = signal(true);

view! {
    <Show when=move || show_alert.get()>
        <Alert color={Color::Warning}>
            <AlertIcon>"⚠"</AlertIcon>
            <AlertContent>
                "This is a dismissible alert with important information."
            </AlertContent>
            <AlertActions>
                <button
                    class="btn btn-sm btn-ghost"
                    on:click=move |_| set_show_alert.set(false)
                >
                    "Close"
                </button>
            </AlertActions>
        </Alert>
    </Show>
    <Show when=move || !show_alert.get()>
        <button class="btn btn-sm" on:click=move |_| set_show_alert.set(true)>
            "Reset Alert"
        </button>
    </Show>
}"#
                >
                    <Show when=move || show_alert.get()>
                        <Alert color={Color::Warning}>
                            <AlertIcon>"⚠"</AlertIcon>
                            <AlertContent>"This is a dismissible alert with important information."</AlertContent>
                            <AlertActions>
                                <button
                                    class="btn btn-sm btn-ghost"
                                    on:click=move |_| set_show_alert.set(false)
                                >
                                    "Close"
                                </button>
                            </AlertActions>
                        </Alert>
                    </Show>
                    <Show when=move || !show_alert.get()>
                         <button class="btn btn-sm" on:click=move |_| set_show_alert.set(true)>
                            "Reset Alert"
                        </button>
                    </Show>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive Alert"
                     code=r#"let (alert_color, set_alert_color) = signal("info");

view! {
    <div class="flex gap-2">
        <button class="btn btn-sm" on:click=move |_| set_alert_color.set("info")>"Info"</button>
        <button class="btn btn-sm" on:click=move |_| set_alert_color.set("success")>"Success"</button>
        <button class="btn btn-sm" on:click=move |_| set_alert_color.set("warning")>"Warning"</button>
        <button class="btn btn-sm" on:click=move |_| set_alert_color.set("error")>"Error"</button>
    </div>

    <div class=move || format!("alert alert-{}", alert_color.get()) role="alert">
        {move || match alert_color.get() {
            "info" => "This is an informational message",
            "success" => "Operation completed successfully!",
            "warning" => "Warning: Please check your settings",
            "error" => "Error: Something went wrong!",
            _ => "",
        }}
    </div>
}"#
                     description="Click buttons to change the alert color dynamically."
                >
                    <div class="space-y-4 w-full">
                        <div class="flex gap-2">
                            <button
                                class="btn btn-sm"
                                on:click=move |_| set_alert_color.set("info")
                            >
                                "Info"
                            </button>
                            <button
                                class="btn btn-sm"
                                on:click=move |_| set_alert_color.set("success")
                            >
                                "Success"
                            </button>
                            <button
                                class="btn btn-sm"
                                on:click=move |_| set_alert_color.set("warning")
                            >
                                "Warning"
                            </button>
                            <button
                                class="btn btn-sm"
                                on:click=move |_| set_alert_color.set("error")
                            >
                                "Error"
                            </button>
                        </div>

                        <div class=move || format!("alert alert-{}", alert_color.get()) role="alert">
                            {move || match alert_color.get() {
                                "info" => "This is an informational message",
                                "success" => "Operation completed successfully!",
                                "warning" => "Warning: Please check your settings",
                                "error" => "Error: Something went wrong!",
                                _ => "",
                            }}
                        </div>
                    </div>
                </ComponentPreview>
            </section>

             <section class="space-y-4">
                <ComponentPreview
                    title="Combination Examples"
                     code=r#"<Alert
    color={Color::Success}
    style={AlertStyle::Outline}
    direction={AlertDirection::Vertical}
>
    <AlertIcon>"✓"</AlertIcon>
    <AlertTitle>"Success"</AlertTitle>
    <AlertContent>"Your changes have been saved successfully."</AlertContent>
</Alert>

<Alert
    color={Color::Error}
    style={AlertStyle::Soft}
    direction={AlertDirection::Horizontal}
>
    <AlertIcon>"✗"</AlertIcon>
    <AlertTitle>"Error"</AlertTitle>
    <AlertContent>"Failed to connect to server."</AlertContent>
</Alert>"#
                >
                    <div class="space-y-4 w-full">
                        <div>
                            <h3 class="text-lg font-medium mb-2">"Vertical + Outline + Success"</h3>
                            <Alert
                                color={Color::Success}
                                style={AlertStyle::Outline}
                                direction={AlertDirection::Vertical}
                            >
                                <AlertIcon>"✓"</AlertIcon>
                                <AlertTitle>"Success"</AlertTitle>
                                <AlertContent>"Your changes have been saved successfully."</AlertContent>
                            </Alert>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium mb-2">"Horizontal + Soft + Error"</h3>
                            <Alert
                                color={Color::Error}
                                style={AlertStyle::Soft}
                                direction={AlertDirection::Horizontal}
                            >
                                <AlertIcon>"✗"</AlertIcon>
                                <AlertTitle>"Error"</AlertTitle>
                                <AlertContent>"Failed to connect to server."</AlertContent>
                            </Alert>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
