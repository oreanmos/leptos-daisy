use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StepsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Steps"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <Steps>
                    <Step color=Color::Primary>"Register"</Step>
                    <Step color=Color::Primary>"Choose plan"</Step>
                    <Step>"Purchase"</Step>
                    <Step>"Receive product"</Step>
                </Steps>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <Steps>
                    <Step color=Color::Info>"Info"</Step>
                    <Step color=Color::Success>"Success"</Step>
                    <Step color=Color::Warning>"Warning"</Step>
                    <Step color=Color::Error>"Error"</Step>
                </Steps>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical"</h2>
                <Steps vertical=true>
                    <Step color=Color::Primary>"Start"</Step>
                    <Step color=Color::Primary>"Middle"</Step>
                    <Step>"End"</Step>
                </Steps>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Responsive"</h2>
                <Steps responsive=true>
                    <Step color=Color::Primary>"Register"</Step>
                    <Step color=Color::Primary>"Choose plan"</Step>
                    <Step>"Purchase"</Step>
                    <Step>"Receive product"</Step>
                </Steps>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Data Content"</h2>
                <Steps>
                    <Step data_content="★" color=Color::Primary>"Step 1"</Step>
                    <Step data_content="★" color=Color::Primary>"Step 2"</Step>
                    <Step data_content="★">"Step 3"</Step>
                    <Step data_content="★">"Step 4"</Step>
                </Steps>
            </section>
        </div>
    }
}
