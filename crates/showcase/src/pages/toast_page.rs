use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Toast"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Alert messages positioned at various screen locations."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "toast", type_label: "base", description: "Base class for toast container" },
                    ClassEntry { name: "toast-start", type_label: "modifier", description: "Align toast to the start (left)" },
                    ClassEntry { name: "toast-center", type_label: "modifier", description: "Align toast to the center" },
                    ClassEntry { name: "toast-end", type_label: "modifier", description: "Align toast to the end (right)" },
                    ClassEntry { name: "toast-top", type_label: "modifier", description: "Position toast at the top" },
                    ClassEntry { name: "toast-middle", type_label: "modifier", description: "Position toast at the middle" },
                    ClassEntry { name: "toast-bottom", type_label: "modifier", description: "Position toast at the bottom" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Toast positions"
                    code=r##"<Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::Start}>
    <Alert color={Color::Info}>"Top Start Toast"</Alert>
</Toast>

<Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::Center}>
    <Alert color={Color::Success}>"Top Center Toast"</Alert>
</Toast>

<Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::End}>
    <Alert color={Color::Warning}>"Top End Toast"</Alert>
</Toast>

<Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::Start}>
    <Alert color={Color::Error}>"Bottom Start Toast"</Alert>
</Toast>

<Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::Center}>
    <Alert color={Color::Info}>"Bottom Center Toast"</Alert>
</Toast>

<Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::End}>
    <Alert color={Color::Success}>"Bottom End Toast"</Alert>
</Toast>"##
                >
                    <div class="grid grid-cols-2 gap-4">
                        <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                            <span class="absolute top-2 left-2 text-sm text-base-content/70">"Top Start"</span>
                            <Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::Start}>
                                <Alert color={Color::Info}>"Top Start Toast"</Alert>
                            </Toast>
                        </div>
                        <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                            <span class="absolute top-2 left-2 text-sm text-base-content/70">"Top Center"</span>
                            <Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::Center}>
                                <Alert color={Color::Success}>"Top Center Toast"</Alert>
                            </Toast>
                        </div>
                        <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                            <span class="absolute top-2 left-2 text-sm text-base-content/70">"Top End"</span>
                            <Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::End}>
                                <Alert color={Color::Warning}>"Top End Toast"</Alert>
                            </Toast>
                        </div>
                        <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                            <span class="absolute top-2 left-2 text-sm text-base-content/70">"Bottom Start"</span>
                            <Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::Start}>
                                <Alert color={Color::Error}>"Bottom Start Toast"</Alert>
                            </Toast>
                        </div>
                        <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                            <span class="absolute top-2 left-2 text-sm text-base-content/70">"Bottom Center"</span>
                            <Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::Center}>
                                <Alert color={Color::Info}>"Bottom Center Toast"</Alert>
                            </Toast>
                        </div>
                        <div class="relative h-48 bg-base-200 rounded-lg overflow-hidden">
                            <span class="absolute top-2 left-2 text-sm text-base-content/70">"Bottom End"</span>
                            <Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::End}>
                                <Alert color={Color::Success}>"Bottom End Toast"</Alert>
                            </Toast>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Multiple toasts"
                    code=r##"<Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::End}>
    <Alert color={Color::Info}>"New message received"</Alert>
    <Alert color={Color::Success}>"Changes saved successfully"</Alert>
    <Alert color={Color::Warning}>"Please check your settings"</Alert>
</Toast>"##
                >
                    <div class="relative h-64 bg-base-200 rounded-lg overflow-hidden">
                        <Toast vertical={ToastVertical::Bottom} horizontal={ToastHorizontal::End}>
                            <Alert color={Color::Info}>"New message received"</Alert>
                            <Alert color={Color::Success}>"Changes saved successfully"</Alert>
                            <Alert color={Color::Warning}>"Please check your settings"</Alert>
                        </Toast>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Toast with different alert styles"
                    code=r##"<Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::Center}>
    <Alert color={Color::Info} style={AlertStyle::Outline}>"Information"</Alert>
    <Alert color={Color::Success} style={AlertStyle::Soft}>"Success!"</Alert>
    <Alert color={Color::Warning} style={AlertStyle::Dash}>"Warning"</Alert>
</Toast>"##
                >
                    <div class="relative h-64 bg-base-200 rounded-lg overflow-hidden">
                        <Toast vertical={ToastVertical::Top} horizontal={ToastHorizontal::Center}>
                            <Alert color={Color::Info} style={AlertStyle::Outline}>"Information"</Alert>
                            <Alert color={Color::Success} style={AlertStyle::Soft}>"Success!"</Alert>
                            <Alert color={Color::Warning} style={AlertStyle::Dash}>"Warning"</Alert>
                        </Toast>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
