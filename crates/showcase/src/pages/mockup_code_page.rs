use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupCodePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mockup Code"</h1>

            <section>
                <ComponentPreview
                    title="Basic"
                    code=r#"<MockupCode>
    <MockupCodeLine prefix="$">"npm i daisyui"</MockupCodeLine>
</MockupCode>"#
                >
                    <MockupCode>
                        <MockupCodeLine prefix="$">"npm i daisyui"</MockupCodeLine>
                    </MockupCode>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Without Prefix"
                    code=r#"<MockupCode>
    <MockupCodeLine>"hello world"</MockupCodeLine>
</MockupCode>"#
                >
                    <MockupCode>
                        <MockupCodeLine>"hello world"</MockupCodeLine>
                    </MockupCode>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Multi-line"
                    code=r#"<MockupCode>
    <MockupCodeLine prefix="1">"fn main() {"</MockupCodeLine>
    <MockupCodeLine prefix="2">"    println!(\"Hello\");"</MockupCodeLine>
    <MockupCodeLine prefix="3">"}"</MockupCodeLine>
</MockupCode>"#
                >
                    <MockupCode>
                        <MockupCodeLine prefix="1">"fn main() {"</MockupCodeLine>
                        <MockupCodeLine prefix="2">"    println!(\"Hello\");"</MockupCodeLine>
                        <MockupCodeLine prefix="3">"}"</MockupCodeLine>
                    </MockupCode>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Highlighted Line"
                    code=r#"<MockupCode>
    <MockupCodeLine prefix="1">"Installing..."</MockupCodeLine>
    <MockupCodeLine prefix="2" class="bg-warning text-warning-content">"Error!"</MockupCodeLine>
    <MockupCodeLine prefix="3">"Retrying..."</MockupCodeLine>
</MockupCode>"#
                >
                    <MockupCode>
                        <MockupCodeLine prefix="1">"Installing..."</MockupCodeLine>
                        <MockupCodeLine prefix="2" class="bg-warning text-warning-content">"Error!"</MockupCodeLine>
                        <MockupCodeLine prefix="3">"Retrying..."</MockupCodeLine>
                    </MockupCode>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Colored Lines"
                    code=r#"<MockupCode>
    <MockupCodeLine prefix="~" class="text-success">"success"</MockupCodeLine>
    <MockupCodeLine prefix="~" class="text-warning">"warning"</MockupCodeLine>
    <MockupCodeLine prefix="~" class="text-error">"error"</MockupCodeLine>
</MockupCode>"#
                >
                    <MockupCode>
                        <MockupCodeLine prefix="~" class="text-success">"success"</MockupCodeLine>
                        <MockupCodeLine prefix="~" class="text-warning">"warning"</MockupCodeLine>
                        <MockupCodeLine prefix="~" class="text-error">"error"</MockupCodeLine>
                    </MockupCode>
                </ComponentPreview>
            </section>
        </div>
    }
}
