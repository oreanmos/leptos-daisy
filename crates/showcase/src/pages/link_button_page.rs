use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LinkButtonPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Link Button"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "An anchor element styled as a daisyUI button. Use instead of raw "<code>"<a class=\"btn\">"</code>"."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<LinkButton href="/example" color=Color::Primary>"Primary"</LinkButton>
    <LinkButton href="/example" color=Color::Secondary>"Secondary"</LinkButton>"##
                >
                    <div class="flex flex-wrap gap-2">
                        <LinkButton href="/example">"Default"</LinkButton>
                        <LinkButton href="/example" color=Color::Primary>"Primary"</LinkButton>
                        <LinkButton href="/example" color=Color::Secondary>"Secondary"</LinkButton>
                        <LinkButton href="/example" color=Color::Accent>"Accent"</LinkButton>
                        <LinkButton href="/example" color=Color::Info>"Info"</LinkButton>
                        <LinkButton href="/example" color=Color::Success>"Success"</LinkButton>
                        <LinkButton href="/example" color=Color::Warning>"Warning"</LinkButton>
                        <LinkButton href="/example" color=Color::Error>"Error"</LinkButton>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Variants"
                    code=r##"<LinkButton href="/example" variant=Variant::Outline>"Outline"</LinkButton>
    <LinkButton href="/example" variant=Variant::Ghost>"Ghost"</LinkButton>"##
                >
                    <div class="flex flex-wrap gap-2">
                        <LinkButton href="/example" color=Color::Primary variant=Variant::Outline>"Outline"</LinkButton>
                        <LinkButton href="/example" color=Color::Primary variant=Variant::Ghost>"Ghost"</LinkButton>
                        <LinkButton href="/example" color=Color::Primary variant=Variant::Link>"Link"</LinkButton>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="External Link"
                    code=r##"<LinkButton href="https://example.com" external=true>"Visit Example"</LinkButton>"##
                >
                    <LinkButton href="https://example.com" external=true color=Color::Primary>"Visit Example (opens in new tab)"</LinkButton>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Disabled"
                    code=r##"<LinkButton href="/example" disabled=true>"Disabled"</LinkButton>"##
                >
                    <LinkButton href="/example" disabled=true color=Color::Primary>"Disabled"</LinkButton>
                </ComponentPreview>
            </section>
        </div>
    }
}
