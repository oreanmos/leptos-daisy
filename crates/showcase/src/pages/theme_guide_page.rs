use crate::components::CodeBlock;
use leptos::prelude::*;

#[component]
pub fn ThemeGuidePage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-4xl">
            <h1 class="text-3xl font-bold">"📖 Theme Guide"</h1>
            <p class="text-base-content/70 text-lg">
                "Everything you need to know about using, installing, and creating daisyUI themes."
            </p>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Built-in Themes"</h2>
                <p>
                    "daisyUI v5 ships with " <strong>"35 built-in themes"</strong>
                    ". Enable them all in your CSS with:"
                </p>
                <CodeBlock
                    language="css"
                    code=r#"@import "tailwindcss";
@plugin "daisyui" {
  themes: all;
}"#
                />
                <p>"Or enable only specific themes:"</p>
                <CodeBlock
                    language="css"
                    code=r#"@plugin "daisyui" {
  themes: light --default, dark --prefersdark, cupcake, nord;
}"#
                />
                <div class="alert alert-info">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <span>"Use " <code>"--default"</code> " to mark the default theme and " <code>"--prefersdark"</code> " for the dark mode fallback."</span>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Applying a Theme"</h2>
                <p>
                    "Set the " <code class="badge badge-ghost badge-sm">"data-theme"</code>
                    " attribute on any HTML element. Usually you put it on " <code>"<html>"</code> " for global theming:"
                </p>
                <CodeBlock language="html" code=r#"<html data-theme="dark">"# />
                <p>"You can also scope themes to specific sections:"</p>
                <CodeBlock
                    language="html"
                    code=r#"<div data-theme="cupcake">
  <!-- This section uses cupcake theme -->
</div>"#
                />

                <h3 class="text-lg font-semibold mt-4">"In Leptos (CSR)"</h3>
                <p>"To set the theme programmatically:"</p>
                <CodeBlock
                    language="rs"
                    code=r#"fn set_theme(name: &str) {
    if let Some(el) = document().document_element() {
        let _ = el.set_attribute("data-theme", name);
    }
}"#
                />

                <h3 class="text-lg font-semibold mt-4">"Persisting Theme Choice"</h3>
                <p>"Use localStorage to remember the user's choice:"</p>
                <CodeBlock
                    language="rs"
                    code=r#"// Save
window().local_storage()
    .ok().flatten()
    .map(|s| s.set_item("theme", name));

// Load
window().local_storage()
    .ok().flatten()
    .and_then(|s| s.get_item("theme").ok().flatten())
    .unwrap_or("light".to_string());"#
                />
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Creating Custom Themes"</h2>
                <p>
                    "daisyUI v5 uses the " <code class="badge badge-ghost badge-sm">"@plugin \"daisyui/theme\""</code>
                    " CSS directive. Add this to your main stylesheet:"
                </p>
                <CodeBlock
                    language="css"
                    code=r#"@plugin "daisyui/theme" {
  name: "my-custom-theme";
  --color-primary: #570df8;
  --color-primary-content: #ffffff;
  --color-secondary: #f000b8;
  --color-secondary-content: #ffffff;
  --color-accent: #37cdbe;
  --color-accent-content: #163835;
  --color-neutral: #3d4451;
  --color-neutral-content: #ffffff;
  --color-base-100: #ffffff;
  --color-base-200: #f2f2f2;
  --color-base-300: #e5e6e6;
  --color-base-content: #1f2937;
  --radius-selector: 0.5rem;
  --radius-field: 0.5rem;
  --radius-box: 0.5rem;
  --border: 1px;
  --depth: 1;
  --noise: 0;
}"#
                />
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Built-In Terminal Theme (Library)"</h2>
                <p>
                    "leptos-daisyui ships a reusable terminal theme helper. "
                    "Inject the CSS once with "
                    <code>"TerminalThemeStyles"</code>
                    ", then wrap your app (or a section) with "
                    <code>"TerminalThemeShell"</code>
                    "."
                </p>
                <CodeBlock
                    language="rs"
                    code=r#"use leptos_daisyui::prelude::*;

view! {
  <TerminalThemeStyles />
  <TerminalThemeShell>
    <div class="p-4 space-y-3">
      <Button color=Color::Primary variant=Variant::Outline>"run"</Button>
      <Input placeholder="type a command..." class="input-sm" />
    </div>
  </TerminalThemeShell>
}"#
                />
                <p>
                    "The shell component applies " <code>"data-theme=\"terminal\""</code>
                    ", mono font, text glow, and optional scanlines. Use "
                    <code>"TERMINAL_THEME_NAME"</code>
                    " if you want to set the theme manually."
                </p>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"CSS Variable Reference"</h2>
                <p>"All the CSS custom properties you can set in a daisyUI theme:"</p>

                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>"Variable"</th>
                                <th>"Description"</th>
                                <th>"Preview"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td><code>"--color-primary"</code></td><td>"Primary brand color"</td><td><div class="bg-primary w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-primary-content"</code></td><td>"Text on primary"</td><td><div class="bg-primary text-primary-content w-16 h-4 rounded text-xs flex items-center justify-center">"Text"</div></td></tr>
                            <tr><td><code>"--color-secondary"</code></td><td>"Secondary brand color"</td><td><div class="bg-secondary w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-accent"</code></td><td>"Accent highlight"</td><td><div class="bg-accent w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-neutral"</code></td><td>"Neutral / dark"</td><td><div class="bg-neutral w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-base-100"</code></td><td>"Page background"</td><td><div class="bg-base-100 border w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-base-200"</code></td><td>"Slightly darker bg"</td><td><div class="bg-base-200 w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-base-300"</code></td><td>"Borders / dividers"</td><td><div class="bg-base-300 w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-base-content"</code></td><td>"Default text color"</td><td><span class="text-base-content font-bold">"Aa"</span></td></tr>
                            <tr><td><code>"--color-info"</code></td><td>"Info state"</td><td><div class="bg-info w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-success"</code></td><td>"Success state"</td><td><div class="bg-success w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-warning"</code></td><td>"Warning state"</td><td><div class="bg-warning w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--color-error"</code></td><td>"Error state"</td><td><div class="bg-error w-6 h-4 rounded"></div></td></tr>
                            <tr><td><code>"--radius-selector"</code></td><td>"Radius for small elements"</td><td></td></tr>
                            <tr><td><code>"--radius-field"</code></td><td>"Radius for form fields"</td><td></td></tr>
                            <tr><td><code>"--radius-box"</code></td><td>"Radius for cards/boxes"</td><td></td></tr>
                            <tr><td><code>"--border"</code></td><td>"Border width"</td><td></td></tr>
                            <tr><td><code>"--depth"</code></td><td>"Shadow depth (0, 1, 2)"</td><td></td></tr>
                            <tr><td><code>"--noise"</code></td><td>"Background texture (0 or 1)"</td><td></td></tr>
                        </tbody>
                    </table>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Tips & Best Practices"</h2>
                <ul class="list-disc list-inside space-y-2">
                    <li>"Always define both a color and its " <code>"-content"</code> " counterpart for readability."</li>
                    <li>"Use oklch() color format for perceptually uniform colors if possible."</li>
                    <li>"Set " <code>"--default"</code> " on your light theme and " <code>"--prefersdark"</code> " on your dark theme for automatic OS preference matching."</li>
                    <li>"Test your theme with all component types — buttons, forms, alerts, cards — to ensure contrast is sufficient."</li>
                    <li>"The Theme Creator page lets you design and export themes visually."</li>
                    <li>"Scoped themes (via " <code>"data-theme"</code> " on a " <code>"<div>"</code> ") allow mixing themes on the same page."</li>
                </ul>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Tailwind v4 Source Scanning"</h2>
                <p>
                    "If you use leptos-daisyui as a separate crate, Tailwind v4 might not scan its classes automatically. "
                    "Add an " <code>"@source"</code> " directive pointing to the crate source:"
                </p>
                <CodeBlock
                    language="css"
                    code=r#"@source "../../crates/leptos-daisyui/src";
    @source "../src";  /* your app source */"#
                />
                <p>"This ensures Tailwind includes all utility classes used by library components."</p>
            </section>

            <section class="flex flex-wrap gap-3">
                <a href="/themes" class="btn btn-primary btn-outline">"🎨 Theme Showcase"</a>
                <a href="/themes/creator" class="btn btn-secondary btn-outline">"🔧 Theme Creator"</a>
                <a href="https://daisyui.com/docs/themes/" target="_blank" class="btn btn-ghost btn-outline">"📚 daisyUI Docs ↗"</a>
            </section>
        </div>
    }
}
