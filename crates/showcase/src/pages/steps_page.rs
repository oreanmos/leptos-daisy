use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StepsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Steps"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Steps"</h2>
                <Steps
                    steps={vec![
                        StepData::new("Register"),
                        StepData::new("Choose Plan"),
                        StepData::new("Payment"),
                        StepData::new("Complete"),
                    ]}
                    current_step={1usize}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Steps"</h2>
                <div class="h-96">
                    <Steps
                        steps={vec![
                            StepData::new("Step 1"),
                            StepData::new("Step 2"),
                            StepData::new("Step 3"),
                            StepData::new("Step 4"),
                        ]}
                        current_step={2usize}
                        orientation={Orientation::Vertical}
                    />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Descriptions"</h2>
                <Steps
                    steps={vec![
                        StepData::with_description("Register", "Create your account"),
                        StepData::with_description("Choose Plan", "Select a subscription"),
                        StepData::with_description("Payment", "Enter payment details"),
                        StepData::with_description("Complete", "Start using the service"),
                    ]}
                    current_step={2usize}
                />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Different Colors"</h2>
                <div class="space-y-4">
                    <Steps
                        steps={vec![
                            StepData::new("Step 1"),
                            StepData::new("Step 2"),
                            StepData::new("Step 3"),
                        ]}
                        current_step={1usize}
                        color={Color::Primary}
                    />
                    <Steps
                        steps={vec![
                            StepData::new("Step 1"),
                            StepData::new("Step 2"),
                            StepData::new("Step 3"),
                        ]}
                        current_step={1usize}
                        color={Color::Secondary}
                    />
                    <Steps
                        steps={vec![
                            StepData::new("Step 1"),
                            StepData::new("Step 2"),
                            StepData::new("Step 3"),
                        ]}
                        current_step={1usize}
                        color={Color::Accent}
                    />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Completed Steps"</h2>
                <Steps
                    steps={vec![
                        StepData::new("Cart"),
                        StepData::new("Shipping"),
                        StepData::new("Payment"),
                        StepData::new("Review"),
                        StepData::new("Confirmation"),
                    ]}
                    current_step={3usize}
                />
            </section>
        </div>
    }
}
