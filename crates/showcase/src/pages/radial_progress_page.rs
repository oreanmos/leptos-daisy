use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RadialProgressPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Radial Progress"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=0.0>"0%"</RadialProgress>
                    <RadialProgress value=25.0>"25%"</RadialProgress>
                    <RadialProgress value=50.0>"50%"</RadialProgress>
                    <RadialProgress value=75.0>"75%"</RadialProgress>
                    <RadialProgress value=100.0>"100%"</RadialProgress>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=70.0 color=Color::Primary>"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Secondary>"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Accent>"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Info>"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Success>"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Warning>"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Error>"70%"</RadialProgress>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Background Color"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=70.0 color=Color::Primary class="bg-primary text-primary-content border-4 border-primary">"70%"</RadialProgress>
                    <RadialProgress value=70.0 color=Color::Success class="bg-success text-success-content border-4 border-success">"70%"</RadialProgress>
                </div>
            </section>
        </div>
    }
}
