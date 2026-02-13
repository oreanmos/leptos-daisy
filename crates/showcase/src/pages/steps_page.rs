use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StepsPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Steps"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Visual progress indicators for multi-step processes with color states and layouts."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "steps", type_label: "base", description: "Base steps container" },
                    ClassEntry { name: "step", type_label: "base part", description: "Individual step" },
                    ClassEntry { name: "step-primary", type_label: "color", description: "Primary color step" },
                    ClassEntry { name: "step-secondary", type_label: "color", description: "Secondary color step" },
                    ClassEntry { name: "step-accent", type_label: "color", description: "Accent color step" },
                    ClassEntry { name: "step-info", type_label: "color", description: "Info color step" },
                    ClassEntry { name: "step-success", type_label: "color", description: "Success color step" },
                    ClassEntry { name: "step-warning", type_label: "color", description: "Warning color step" },
                    ClassEntry { name: "step-error", type_label: "color", description: "Error color step" },
                    ClassEntry { name: "step-neutral", type_label: "color", description: "Neutral color step" },
                    ClassEntry { name: "steps-horizontal", type_label: "modifier", description: "Horizontal layout" },
                    ClassEntry { name: "steps-vertical", type_label: "modifier", description: "Vertical layout" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r##"<Steps>
    <Step color=Color::Primary>"Register"</Step>
    <Step color=Color::Primary>"Choose plan"</Step>
    <Step>"Purchase"</Step>
    <Step>"Receive product"</Step>
</Steps>"##
                >
                    <Steps>
                        <Step color=Color::Primary>"Register"</Step>
                        <Step color=Color::Primary>"Choose plan"</Step>
                        <Step>"Purchase"</Step>
                        <Step>"Receive product"</Step>
                    </Steps>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<Steps>
    <Step color=Color::Info>"Info"</Step>
    <Step color=Color::Success>"Success"</Step>
    <Step color=Color::Warning>"Warning"</Step>
    <Step color=Color::Error>"Error"</Step>
</Steps>"##
                >
                    <Steps>
                        <Step color=Color::Info>"Info"</Step>
                        <Step color=Color::Success>"Success"</Step>
                        <Step color=Color::Warning>"Warning"</Step>
                        <Step color=Color::Error>"Error"</Step>
                    </Steps>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Vertical"
                    code=r##"<Steps vertical=true>
    <Step color=Color::Primary>"Start"</Step>
    <Step color=Color::Primary>"Middle"</Step>
    <Step>"End"</Step>
</Steps>"##
                >
                    <Steps vertical=true>
                        <Step color=Color::Primary>"Start"</Step>
                        <Step color=Color::Primary>"Middle"</Step>
                        <Step>"End"</Step>
                    </Steps>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Responsive"
                    code=r##"<Steps responsive=true>
    <Step color=Color::Primary>"Register"</Step>
    <Step color=Color::Primary>"Choose plan"</Step>
    <Step>"Purchase"</Step>
    <Step>"Receive product"</Step>
</Steps>"##
                >
                    <Steps responsive=true>
                        <Step color=Color::Primary>"Register"</Step>
                        <Step color=Color::Primary>"Choose plan"</Step>
                        <Step>"Purchase"</Step>
                        <Step>"Receive product"</Step>
                    </Steps>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Data Content"
                    code=r##"<Steps>
    <Step data_content="★" color=Color::Primary>"Step 1"</Step>
    <Step data_content="★" color=Color::Primary>"Step 2"</Step>
    <Step data_content="★">"Step 3"</Step>
    <Step data_content="★">"Step 4"</Step>
</Steps>"##
                >
                    <Steps>
                        <Step data_content="★" color=Color::Primary>"Step 1"</Step>
                        <Step data_content="★" color=Color::Primary>"Step 2"</Step>
                        <Step data_content="★">"Step 3"</Step>
                        <Step data_content="★">"Step 4"</Step>
                    </Steps>
                </ComponentPreview>
            </section>
        </div>
    }
}
