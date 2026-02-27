use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StatusIndicatorPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Status Indicator"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A badge-style status indicator with optional colored dot."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r#"<StatusIndicator label="Active" color=Color::Success />
<StatusIndicator label="Warning" color=Color::Warning />
<StatusIndicator label="Error" color=Color::Error />"#
                >
                    <div class="flex flex-wrap gap-2">
                        <StatusIndicator label="Default" />
                        <StatusIndicator label="Active" color=Color::Success />
                        <StatusIndicator label="Pending" color=Color::Warning />
                        <StatusIndicator label="Error" color=Color::Error />
                        <StatusIndicator label="Info" color=Color::Info />
                        <StatusIndicator label="Primary" color=Color::Primary />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Dot"
                    code=r#"<StatusIndicator label="Online" color=Color::Success dot=true />
<StatusIndicator label="Offline" color=Color::Error dot=true />"#
                >
                    <div class="flex flex-wrap gap-2">
                        <StatusIndicator label="Online" color=Color::Success dot=true />
                        <StatusIndicator label="Away" color=Color::Warning dot=true />
                        <StatusIndicator label="Offline" color=Color::Error dot=true />
                        <StatusIndicator label="Unknown" dot=true />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<StatusIndicator label="Small" size=Size::Small />
<StatusIndicator label="Large" size=Size::Large />"#
                >
                    <div class="flex flex-wrap items-center gap-2">
                        <StatusIndicator label="XS" size=Size::ExtraSmall color=Color::Success dot=true />
                        <StatusIndicator label="SM" size=Size::Small color=Color::Success dot=true />
                        <StatusIndicator label="MD" color=Color::Success dot=true />
                        <StatusIndicator label="LG" size=Size::Large color=Color::Success dot=true />
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
