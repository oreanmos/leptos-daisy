use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupCodePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mockup Code"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic"</h2>
                <MockupCode>
                    <MockupCodeLine prefix="$">"npm i daisyui"</MockupCodeLine>
                </MockupCode>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Without Prefix"</h2>
                <MockupCode>
                    <MockupCodeLine>"hello world"</MockupCodeLine>
                </MockupCode>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Multi-line"</h2>
                <MockupCode>
                    <MockupCodeLine prefix="1">"fn main() {"</MockupCodeLine>
                    <MockupCodeLine prefix="2">"    println!(\"Hello\");"</MockupCodeLine>
                    <MockupCodeLine prefix="3">"}"</MockupCodeLine>
                </MockupCode>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Highlighted Line"</h2>
                <MockupCode>
                    <MockupCodeLine prefix="1">"Installing..."</MockupCodeLine>
                    <MockupCodeLine prefix="2" class="bg-warning text-warning-content">"Error!"</MockupCodeLine>
                    <MockupCodeLine prefix="3">"Retrying..."</MockupCodeLine>
                </MockupCode>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colored Lines"</h2>
                <MockupCode>
                    <MockupCodeLine prefix="~" class="text-success">"success"</MockupCodeLine>
                    <MockupCodeLine prefix="~" class="text-warning">"warning"</MockupCodeLine>
                    <MockupCodeLine prefix="~" class="text-error">"error"</MockupCodeLine>
                </MockupCode>
            </section>
        </div>
    }
}
