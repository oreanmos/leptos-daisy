use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupCodePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Mockup Code"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Code Block"</h2>
                <MockupCode lines={vec![
                    CodeLine::new("npm install leptos-daisyui"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Line Numbers"</h2>
                <MockupCode lines={vec![
                    CodeLine::with_prefix("cargo new my-app", "$"),
                    CodeLine::with_prefix("Created binary (application) 'my-app' package", ">"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Syntax Highlighting"</h2>
                <MockupCode lines={vec![
                    CodeLine::with_prefix_and_color("use leptos::prelude::*;", "1", "text-primary"),
                    CodeLine::with_prefix_and_color("use leptos_daisyui::prelude::*;", "2", "text-primary"),
                    CodeLine::with_prefix("", "3"),
                    CodeLine::with_prefix_and_color("#[component]", "4", "text-secondary"),
                    CodeLine::with_prefix_and_color("fn App() -> impl IntoView {", "5", "text-accent"),
                    CodeLine::with_prefix("    view! {", "6"),
                    CodeLine::with_prefix_and_color("        <Button>\"Click me\"</Button>", "7", "text-info"),
                    CodeLine::with_prefix("    }", "8"),
                    CodeLine::with_prefix_and_color("}", "9", "text-accent"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Command Output"</h2>
                <MockupCode lines={vec![
                    CodeLine::with_prefix_and_color("cargo build --release", "$", "text-success"),
                    CodeLine::with_prefix("   Compiling leptos-daisyui v0.1.0", ">"),
                    CodeLine::with_prefix("   Compiling showcase v0.1.0", ">"),
                    CodeLine::with_prefix("    Finished release [optimized] target(s) in 45.23s", ">"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Error Output"</h2>
                <MockupCode lines={vec![
                    CodeLine::with_prefix_and_color("cargo test", "$", "text-error"),
                    CodeLine::with_prefix("   Compiling myapp v0.1.0", ">"),
                    CodeLine::with_prefix_and_color("error[E0425]: cannot find function `unknown` in this scope", ">", "text-error"),
                    CodeLine::with_prefix(" --> src/main.rs:5:5", ">"),
                    CodeLine::with_prefix("   |", ">"),
                    CodeLine::with_prefix("5 |     unknown();", ">"),
                    CodeLine::with_prefix("  |     ^^^^^^^ not found in this scope", ">"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Installation Commands"</h2>
                <div class="space-y-4">
                    <MockupCode lines={vec![
                        CodeLine::with_prefix_and_color("cargo add leptos", "$", "text-primary"),
                        CodeLine::with_prefix_and_color("cargo add leptos-daisyui", "$", "text-primary"),
                    ]} />

                    <MockupCode lines={vec![
                        CodeLine::with_prefix_and_color("npm install -D tailwindcss", "$", "text-secondary"),
                        CodeLine::with_prefix_and_color("npm install -D daisyui@latest", "$", "text-secondary"),
                    ]} />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Configuration Example"</h2>
                <MockupCode lines={vec![
                    CodeLine::with_prefix_and_color("// tailwind.config.js", "1", "text-warning"),
                    CodeLine::with_prefix("module.exports = {", "2"),
                    CodeLine::with_prefix("  content: ['./src/**/*.rs'],", "3"),
                    CodeLine::with_prefix("  theme: {", "4"),
                    CodeLine::with_prefix("    extend: {},", "5"),
                    CodeLine::with_prefix("  },", "6"),
                    CodeLine::with_prefix("  plugins: [require('daisyui')],", "7"),
                    CodeLine::with_prefix("}", "8"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Custom Data Prefix"</h2>
                <MockupCode lines={vec![
                    CodeLine::with_prefix_and_color("main", "git:", "text-info"),
                    CodeLine::with_prefix_and_color("developer", "user:", "text-success"),
                    CodeLine::with_prefix_and_color("localhost", "host:", "text-warning"),
                    CodeLine::with_prefix_and_color("~/projects/myapp", "path:", "text-accent"),
                ]} />
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Raw Content"</h2>
                <MockupCodeRaw>
                    <pre data-prefix="$"><code>"cat README.md"</code></pre>
                    <pre data-prefix=">"><code>"leptos-daisyui"</code></pre>
                    <pre data-prefix=">"><code>"A Leptos component library for daisyUI"</code></pre>
                </MockupCodeRaw>
            </section>
        </div>
    }
}
