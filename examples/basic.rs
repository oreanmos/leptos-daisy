//! Basic Example
//!
//! This example demonstrates how to use the basic DaisyUI components in Leptos.

use leptos::prelude::*;
use leptos::*;
use leptos_daisy::components::*;

/// Basic example component
#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div class="p-4 space-y-4">
            <h1 class="text-2xl font-bold">"Leptos DaisyUI Components"</h1>

            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"Buttons"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button variant=ButtonVariant::Primary>
                        "Primary Button"
                    </Button>
                    <Button variant=ButtonVariant::Secondary>
                        "Secondary Button"
                    </Button>
                    <Button variant=ButtonVariant::Success outline=true>
                        "Success Outline"
                    </Button>
                    <Button variant=ButtonVariant::Warning ghost=true>
                        "Warning Ghost"
                    </Button>
                    <Button variant=ButtonVariant::Error size=ButtonSize::Small>
                        "Error Small"
                    </Button>
                </div>
            </div>

            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"Alerts"</h2>
                <div class="space-y-2">
                    <Alert variant=AlertVariant::Info icon=true>
                        <strong>"Info:"</strong> "This is an informational message."
                    </Alert>
                    <Alert variant=AlertVariant::Success icon=true>
                        <strong>"Success:"</strong> "Operation completed successfully!"
                    </Alert>
                    <Alert variant=AlertVariant::Warning icon=true>
                        <strong>"Warning:"</strong> "Please check your input."
                    </Alert>
                    <Alert variant=AlertVariant::Error icon=true>
                        <strong>"Error:"</strong> "Something went wrong!"
                    </Alert>
                </div>
            </div>

            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"Cards"</h2>
                <div class="flex flex-wrap gap-4">
                    <Card variant=CardVariant::Bordered class=Some("w-64".to_string())>
                        <CardBody>
                            <CardTitle>"Card Title"</CardTitle>
                            <p>"This is a bordered card with some content."</p>
                            <CardActions>
                                <Button variant=ButtonVariant::Primary size=ButtonSize::Small>
                                    "Action"
                                </Button>
                            </CardActions>
                        </CardBody>
                    </Card>

                    <Card variant=CardVariant::Compact class=Some("w-64".to_string())>
                        <CardBody>
                            <CardTitle>"Compact Card"</CardTitle>
                            <p>"This card has a compact layout."</p>
                        </CardBody>
                    </Card>
                </div>
            </div>
        </div>
    }
}
