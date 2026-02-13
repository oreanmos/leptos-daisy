use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let (value, set_value) = signal(30.0_f64);
    let progress_value = Signal::derive(move || value.get());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Progress"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Determinate and indeterminate progress bars with full color coverage."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "progress", type_label: "base", description: "Base class for progress bar" },
                    ClassEntry { name: "progress-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "progress-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "progress-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "progress-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "progress-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "progress-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "progress-error", type_label: "color", description: "Error color" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r#"<Progress value=40.0 />
<Progress value=50.0 color={Color::Primary} />
<Progress value=60.0 color={Color::Secondary} />
<Progress value=70.0 color={Color::Accent} />
<Progress value=80.0 color={Color::Success} />
<Progress value=90.0 color={Color::Warning} />
<Progress value=100.0 color={Color::Error} />"#
                >
                    <div class="flex flex-col gap-3 max-w-md">
                        <Progress value=40.0 />
                        <Progress value=50.0 color={Color::Primary} />
                        <Progress value=60.0 color={Color::Secondary} />
                        <Progress value=70.0 color={Color::Accent} />
                        <Progress value=80.0 color={Color::Success} />
                        <Progress value=90.0 color={Color::Warning} />
                        <Progress value=100.0 color={Color::Error} />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Custom ranges"
                    code=r#"<Progress value=7.0 max=10.0 color={Color::Success} />
    <Progress value=3.0 max=5.0 color={Color::Warning} />"#
                >
                    <div class="space-y-3 max-w-md">
                        <Progress value=7.0 max=10.0 color={Color::Success} />
                        <Progress value=3.0 max=5.0 color={Color::Warning} />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Indeterminate"
                    code=r#"<Progress color={Color::Primary} />
    <Progress color={Color::Secondary} class="w-full" />"#
                >
                    <div class="space-y-3 max-w-md">
                        <Progress color={Color::Primary} />
                        <Progress color={Color::Secondary} class="w-full" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive progress"
                    code=r#"let (value, set_value) = signal(30.0_f64);
let progress_value = Signal::derive(move || value.get());

view! {
    <Progress value=progress_value max=100.0 color={Color::Primary} />
    <input
        type="range"
        min="0"
        max="100"
        value=move || value.get().to_string()
        class="range range-primary"
        on:input=move |ev| {
            let next = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
            set_value.set(next);
        }
    />
    <p class="text-sm text-base-content/70">
        {move || format!("{:.0}%", value.get())}
    </p>
}"#
                >
                    <div class="space-y-3 max-w-md">
                        <Progress value=progress_value max=100.0 color={Color::Primary} />
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value=move || value.get().to_string()
                            class="range range-primary"
                            on:input=move |ev| {
                                let next = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_value.set(next);
                            }
                        />
                        <p class="text-sm text-base-content/70">
                            {move || format!("{:.0}%", value.get())}
                        </p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
