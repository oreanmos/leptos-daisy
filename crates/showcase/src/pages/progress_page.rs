use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Progress"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-col gap-2 max-w-md">
                    <Progress value=40.0 />
                    <Progress value=50.0 color={Color::Primary} />
                    <Progress value=60.0 color={Color::Secondary} />
                    <Progress value=70.0 color={Color::Accent} />
                    <Progress value=80.0 color={Color::Success} />
                    <Progress value=90.0 color={Color::Warning} />
                    <Progress value=100.0 color={Color::Error} />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Indeterminate"</h2>
                <Progress color={Color::Primary} class="max-w-md" />
            </section>
        </div>
    }
}
