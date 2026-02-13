use crate::components::component_preview::ComponentPreview;
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
