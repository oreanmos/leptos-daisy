use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LinkPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Link"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Styled hyperlinks with color variants, hover effects, and external link handling."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "link", type_label: "base", description: "Base link class" },
                    ClassEntry { name: "link-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "link-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "link-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "link-neutral", type_label: "color", description: "Neutral color" },
                    ClassEntry { name: "link-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "link-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "link-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "link-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "link-hover", type_label: "modifier", description: "Underline on hover only" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Default Link"
                    code=r##"<p>
    "This is a paragraph with a "
    <Link href="/">"default link"</Link>
    " inside it."
</p>"##
                >
                    <p>
                        "This is a paragraph with a "
                        <Link href="/">"default link"</Link>
                        " inside it."
                    </p>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Link Colors"
                    code=r##"<Link href="/" color={Color::Primary}>"Primary"</Link>
<Link href="/" color={Color::Secondary}>"Secondary"</Link>
<Link href="/" color={Color::Accent}>"Accent"</Link>
<Link href="/" color={Color::Neutral}>"Neutral"</Link>
<Link href="/" color={Color::Info}>"Info"</Link>
<Link href="/" color={Color::Success}>"Success"</Link>
<Link href="/" color={Color::Warning}>"Warning"</Link>
<Link href="/" color={Color::Error}>"Error"</Link>"##
                >
                    <div class="flex flex-wrap gap-4">
                        <Link href="/" color={Color::Primary}>"Primary"</Link>
                        <Link href="/" color={Color::Secondary}>"Secondary"</Link>
                        <Link href="/" color={Color::Accent}>"Accent"</Link>
                        <Link href="/" color={Color::Neutral}>"Neutral"</Link>
                        <Link href="/" color={Color::Info}>"Info"</Link>
                        <Link href="/" color={Color::Success}>"Success"</Link>
                        <Link href="/" color={Color::Warning}>"Warning"</Link>
                        <Link href="/" color={Color::Error}>"Error"</Link>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Hover Effect"
                    code=r##"<Link href="/" hover=true>"With Hover"</Link>
<Link href="/" hover=true color={Color::Primary}>"Primary Hover"</Link>
<Link href="/" hover=true color={Color::Secondary}>"Secondary Hover"</Link>"##
                >
                    <div class="flex flex-wrap gap-4">
                        <Link href="/" hover=true>"With Hover"</Link>
                        <Link href="/" hover=true color={Color::Primary}>"Primary Hover"</Link>
                        <Link href="/" hover=true color={Color::Secondary}>"Secondary Hover"</Link>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="External Links"
                    code=r##"<Link href="https://example.com" external=true>"External Link"</Link>
<Link href="https://github.com" external=true color={Color::Primary}>"GitHub"</Link>
<Link href="https://docs.rs" external=true color={Color::Secondary} hover=true>"Documentation"</Link>"##
                >
                    <div class="flex flex-wrap gap-4">
                        <Link href="https://example.com" external=true>"External Link"</Link>
                        <Link href="https://github.com" external=true color={Color::Primary}>"GitHub"</Link>
                        <Link href="https://docs.rs" external=true color={Color::Secondary} hover=true>"Documentation"</Link>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
