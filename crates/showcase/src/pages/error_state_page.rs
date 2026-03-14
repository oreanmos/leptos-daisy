use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ErrorStatePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Error State"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "An error display component with a message, optional description, and retry action."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Error"
                    code=r##"<ErrorState message="Something went wrong" />"##
                >
                    <ErrorState message="Something went wrong" />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Description"
                    code=r##"<ErrorState
    message="Failed to load data"
    description="The server returned an unexpected error. Please check your connection."
/>"##
                >
                    <ErrorState
                        message="Failed to load data"
                        description="The server returned an unexpected error. Please check your connection."
                    />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Retry"
                    code=r##"<ErrorState
    message="Network error"
    description="Could not reach the server."
    on_retry=Callback::new(|_| { /* retry logic */ })
/>"##
                >
                    <ErrorState
                        message="Network error"
                        description="Could not reach the server."
                        on_retry=Callback::new(|_| {})
                    />
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Custom Retry Label"
                    code=r##"<ErrorState
    message="Upload failed"
    on_retry=Callback::new(|_| {})
    retry_label="Try Again"
/>"##
                >
                    <ErrorState
                        message="Upload failed"
                        on_retry=Callback::new(|_| {})
                        retry_label="Try Again"
                    />
                </ComponentPreview>
            </section>
        </div>
    }
}
