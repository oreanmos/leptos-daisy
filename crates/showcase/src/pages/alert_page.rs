use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AlertPage() -> impl IntoView {
    // State for reactive alert example
    let (alert_variant, set_alert_variant) = signal(AlertVariant::Info);
    let (show_alert, set_show_alert) = signal(true);

    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Alert"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Variants"</h2>
                <div class="space-y-2">
                    <Alert variant={AlertVariant::Info}>"This is an info alert."</Alert>
                    <Alert variant={AlertVariant::Success}>"This is a success alert."</Alert>
                    <Alert variant={AlertVariant::Warning}>"This is a warning alert."</Alert>
                    <Alert variant={AlertVariant::Error}>"This is an error alert."</Alert>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Styles"</h2>
                <div class="space-y-2">
                    <Alert variant={AlertVariant::Info} style={AlertStyle::Outline}>"Outline info alert"</Alert>
                    <Alert variant={AlertVariant::Success} style={AlertStyle::Dash}>"Dash success alert"</Alert>
                    <Alert variant={AlertVariant::Warning} style={AlertStyle::Soft}>"Soft warning alert"</Alert>
                    <Alert variant={AlertVariant::Error} style={AlertStyle::Soft}>"Soft error alert"</Alert>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Layout Directions"</h2>
                <div class="space-y-4">
                    <div>
                        <h3 class="text-lg font-medium mb-2">"Vertical Layout"</h3>
                        <Alert variant={AlertVariant::Info} direction={AlertDirection::Vertical}>
                            <AlertIcon>"ℹ"</AlertIcon>
                            <AlertTitle>"Info"</AlertTitle>
                            <AlertContent>"This alert uses vertical layout for better readability on mobile devices."</AlertContent>
                        </Alert>
                    </div>

                    <div>
                        <h3 class="text-lg font-medium mb-2">"Horizontal Layout"</h3>
                        <Alert variant={AlertVariant::Success} direction={AlertDirection::Horizontal}>
                            <AlertIcon>"✓"</AlertIcon>
                            <AlertTitle>"Success"</AlertTitle>
                            <AlertContent>"This alert uses horizontal layout for compact display."</AlertContent>
                        </Alert>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Sub-components"</h2>
                <Alert variant={AlertVariant::Info}>
                    <AlertIcon>"ℹ"</AlertIcon>
                    <AlertTitle>"Info"</AlertTitle>
                    <AlertContent>"New software update available."</AlertContent>
                    <AlertActions>
                        <button class="btn btn-sm">"Deny"</button>
                        <button class="btn btn-sm btn-primary">"Accept"</button>
                    </AlertActions>
                </Alert>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Dismissible Alert"</h2>
                <Show when=move || show_alert.get()>
                    <Alert variant={AlertVariant::Warning}>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Reactive Alert"</h2>
                <div class="space-y-4">
                    <div class="flex gap-2">
                        <button
                            class="btn btn-sm"
                            on:click=move |_| set_alert_variant.set(AlertVariant::Info)
                        >
                            "Info"
                        </button>
                        <button
                            class="btn btn-sm"
                            on:click=move |_| set_alert_variant.set(AlertVariant::Success)
                        >
                            "Success"
                        </button>
                        <button
                            class="btn btn-sm"
                            on:click=move |_| set_alert_variant.set(AlertVariant::Warning)
                        >
                            "Warning"
                        </button>
                        <button
                            class="btn btn-sm"
                            on:click=move |_| set_alert_variant.set(AlertVariant::Error)
                        >
                            "Error"
                        </button>
                    </div>

                    <Alert variant={alert_variant.get()}>
                        <AlertContent>
                            {move || match alert_variant.get() {
                                AlertVariant::Info => "This is an informational message",
                                AlertVariant::Success => "Operation completed successfully!",
                                AlertVariant::Warning => "Warning: Please check your settings",
                                AlertVariant::Error => "Error: Something went wrong!",
                            }}
                        </AlertContent>
                    </Alert>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Stacked Alerts"</h2>
                <div class="space-y-2">
                    <Alert variant={AlertVariant::Info} style={AlertStyle::Outline}>
                        <AlertIcon>"ℹ"</AlertIcon>
                        <AlertContent>"New message received"</AlertContent>
                    </Alert>
                    <Alert variant={AlertVariant::Success} style={AlertStyle::Soft}>
                        <AlertIcon>"✓"</AlertIcon>
                        <AlertContent>"Task completed successfully"</AlertContent>
                    </Alert>
                    <Alert variant={AlertVariant::Warning} style={AlertStyle::Dash}>
                        <AlertIcon>"⚠"</AlertIcon>
                        <AlertContent>"Please review your settings"</AlertContent>
                    </Alert>
                    <Alert variant={AlertVariant::Error}>
                        <AlertIcon>"✗"</AlertIcon>
                        <AlertContent>"Critical error occurred"</AlertContent>
                    </Alert>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Combination Examples"</h2>
                <div class="space-y-4">
                    <div>
                        <h3 class="text-lg font-medium mb-2">"Vertical + Outline + Success"</h3>
                        <Alert
                            variant={AlertVariant::Success}
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
                            variant={AlertVariant::Error}
                            style={AlertStyle::Soft}
                            direction={AlertDirection::Horizontal}
                        >
                            <AlertIcon>"✗"</AlertIcon>
                            <AlertTitle>"Error"</AlertTitle>
                            <AlertContent>"Failed to connect to server."</AlertContent>
                        </Alert>
                    </div>
                </div>
            </section>
        </div>
    }
}
