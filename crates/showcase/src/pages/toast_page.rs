use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Toast"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Toast Positions"</h2>
                <div class="grid grid-cols-2 gap-4">
                    <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                        <span class="absolute top-2 left-2 text-sm text-base-content/70">"Top Start"</span>
                        <Toast vertical={VerticalPosition::Top} horizontal={HorizontalPosition::Start}>
                            <Alert variant={AlertVariant::Info}>"Top Start Toast"</Alert>
                        </Toast>
                    </div>
                    <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                        <span class="absolute top-2 left-2 text-sm text-base-content/70">"Top Center"</span>
                        <Toast vertical={VerticalPosition::Top} horizontal={HorizontalPosition::Center}>
                            <Alert variant={AlertVariant::Success}>"Top Center Toast"</Alert>
                        </Toast>
                    </div>
                    <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                        <span class="absolute top-2 left-2 text-sm text-base-content/70">"Top End"</span>
                        <Toast vertical={VerticalPosition::Top} horizontal={HorizontalPosition::End}>
                            <Alert variant={AlertVariant::Warning}>"Top End Toast"</Alert>
                        </Toast>
                    </div>
                    <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                        <span class="absolute top-2 left-2 text-sm text-base-content/70">"Bottom Start"</span>
                        <Toast vertical={VerticalPosition::Bottom} horizontal={HorizontalPosition::Start}>
                            <Alert variant={AlertVariant::Error}>"Bottom Start Toast"</Alert>
                        </Toast>
                    </div>
                    <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                        <span class="absolute top-2 left-2 text-sm text-base-content/70">"Bottom Center"</span>
                        <Toast vertical={VerticalPosition::Bottom} horizontal={HorizontalPosition::Center}>
                            <Alert variant={AlertVariant::Info}>"Bottom Center Toast"</Alert>
                        </Toast>
                    </div>
                    <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                        <span class="absolute top-2 left-2 text-sm text-base-content/70">"Bottom End"</span>
                        <Toast vertical={VerticalPosition::Bottom} horizontal={HorizontalPosition::End}>
                            <Alert variant={AlertVariant::Success}>"Bottom End Toast"</Alert>
                        </Toast>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Multiple Toasts"</h2>
                <div class="relative h-64 bg-base-200 rounded-lg overflow-hidden">
                    <Toast vertical={VerticalPosition::Bottom} horizontal={HorizontalPosition::End}>
                        <Alert variant={AlertVariant::Info}>"New message received"</Alert>
                        <Alert variant={AlertVariant::Success}>"Changes saved successfully"</Alert>
                        <Alert variant={AlertVariant::Warning}>"Please check your settings"</Alert>
                    </Toast>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Toast with Different Alert Styles"</h2>
                <div class="relative h-64 bg-base-200 rounded-lg overflow-hidden">
                    <Toast vertical={VerticalPosition::Top} horizontal={HorizontalPosition::Center}>
                        <Alert variant={AlertVariant::Info} style={AlertStyle::Outline}>"Information"</Alert>
                        <Alert variant={AlertVariant::Success} style={AlertStyle::Soft}>"Success!"</Alert>
                        <Alert variant={AlertVariant::Warning} style={AlertStyle::Dash}>"Warning"</Alert>
                    </Toast>
                </div>
            </section>
        </div>
    }
}
