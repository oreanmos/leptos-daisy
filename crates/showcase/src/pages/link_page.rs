use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LinkPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Link"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Default Link"</h2>
                <p>
                    "This is a paragraph with a "
                    <Link href="/">"default link"</Link>
                    " inside it."
                </p>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Link Colors"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Hover Effect"</h2>
                <div class="flex flex-wrap gap-4">
                    <Link href="/" hover=true>"With Hover"</Link>
                    <Link href="/" hover=true color={Color::Primary}>"Primary Hover"</Link>
                    <Link href="/" hover=true color={Color::Secondary}>"Secondary Hover"</Link>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"External Links"</h2>
                <div class="flex flex-wrap gap-4">
                    <Link href="https://example.com" external=true>"External Link"</Link>
                    <Link href="https://github.com" external=true color={Color::Primary}>"GitHub"</Link>
                    <Link href="https://docs.rs" external=true color={Color::Secondary} hover=true>"Documentation"</Link>
                </div>
            </section>
        </div>
    }
}
