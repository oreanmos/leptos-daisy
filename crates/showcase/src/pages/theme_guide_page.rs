use leptos::prelude::*;

#[component]
pub fn ThemeGuidePage() -> impl IntoView {
    view! {
        <div class="space-y-8 max-w-4xl">
            <h1 class="text-3xl font-bold">"📖 Theme Guide"</h1>
            <p class="text-base-content/70 text-lg">
                "Everything you need to know about using, installing, and creating daisyUI themes."
            </p>

            // --- Section 1: Built-in Themes ---
            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Built-in Themes"</h2>
                <p>
                    "daisyUI v5 ships with " <strong>"35 built-in themes"</strong>
                    ". Enable them all in your CSS with:"
                </p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"@import \"tailwindcss\";"</code></pre>
                    <pre data-prefix="2"><code>"@plugin \"daisyui\" {"</code></pre>
                    <pre data-prefix="3"><code>"  themes: all;"</code></pre>
                    <pre data-prefix="4"><code>"}"</code></pre>
                </div>
                <p>"Or enable only specific themes:"</p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"@plugin \"daisyui\" {"</code></pre>
                    <pre data-prefix="2"><code>"  themes: light --default, dark --prefersdark, cupcake, nord;"</code></pre>
                    <pre data-prefix="3"><code>"}"</code></pre>
                </div>
                <div class="alert alert-info">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <span>"Use " <code>"--default"</code> " to mark the default theme and " <code>"--prefersdark"</code> " for the dark mode fallback."</span>
                </div>
            </section>

            // --- Section 2: Applying a Theme ---
            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Applying a Theme"</h2>
                <p>
                    "Set the " <code class="badge badge-ghost badge-sm">"data-theme"</code>
                    " attribute on any HTML element. Usually you put it on " <code>"<html>"</code> " for global theming:"
                </p>
                <div class="mockup-code text-sm">
                    <pre data-prefix=""><code>"<html data-theme=\"dark\">"</code></pre>
                </div>
                <p>"You can also scope themes to specific sections:"</p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"<div data-theme=\"cupcake\">"</code></pre>
                    <pre data-prefix="2"><code>"  <!-- This section uses cupcake theme -->"</code></pre>
                    <pre data-prefix="3"><code>"</div>"</code></pre>
                </div>

                <h3 class="text-lg font-semibold mt-4">"In Leptos (CSR)"</h3>
                <p>"To set the theme programmatically:"</p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"fn set_theme(name: &str) {"</code></pre>
                    <pre data-prefix="2"><code>"    if let Some(el) = document().document_element() {"</code></pre>
                    <pre data-prefix="3"><code>"        let _ = el.set_attribute(\"data-theme\", name);"</code></pre>
                    <pre data-prefix="4"><code>"    }"</code></pre>
                    <pre data-prefix="5"><code>"}"</code></pre>
                </div>

                <h3 class="text-lg font-semibold mt-4">"Persisting Theme Choice"</h3>
                <p>"Use localStorage to remember the user's choice:"</p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"// Save"</code></pre>
                    <pre data-prefix="2"><code>"window().local_storage()"</code></pre>
                    <pre data-prefix="3"><code>"    .ok().flatten()"</code></pre>
                    <pre data-prefix="4"><code>"    .map(|s| s.set_item(\"theme\", name));"</code></pre>
                    <pre data-prefix="5"><code>""</code></pre>
                    <pre data-prefix="6"><code>"// Load"</code></pre>
                    <pre data-prefix="7"><code>"window().local_storage()"</code></pre>
                    <pre data-prefix="8"><code>"    .ok().flatten()"</code></pre>
                    <pre data-prefix="9"><code>"    .and_then(|s| s.get_item(\"theme\").ok().flatten())"</code></pre>
                    <pre data-prefix="10"><code>"    .unwrap_or(\"light\".to_string());"</code></pre>
                </div>
            </section>

            // --- Section 3: Creating Custom Themes ---
            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Creating Custom Themes"</h2>
                <p>
                    "daisyUI v5 uses the " <code class="badge badge-ghost badge-sm">"@plugin \"daisyui/theme\""</code>
                    " CSS directive. Add this to your main stylesheet:"
                </p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"@plugin \"daisyui/theme\" {"</code></pre>
                    <pre data-prefix="2"><code>"  name: \"my-custom-theme\";"</code></pre>
                    <pre data-prefix="3"><code>"  --color-primary: #570df8;"</code></pre>
                    <pre data-prefix="4"><code>"  --color-primary-content: #ffffff;"</code></pre>
                    <pre data-prefix="5"><code>"  --color-secondary: #f000b8;"</code></pre>
                    <pre data-prefix="6"><code>"  --color-secondary-content: #ffffff;"</code></pre>
                    <pre data-prefix="7"><code>"  --color-accent: #37cdbe;"</code></pre>
                    <pre data-prefix="8"><code>"  --color-accent-content: #163835;"</code></pre>
                    <pre data-prefix="9"><code>"  --color-neutral: #3d4451;"</code></pre>
                    <pre data-prefix="10"><code>"  --color-neutral-content: #ffffff;"</code></pre>
                    <pre data-prefix="11"><code>"  --color-base-100: #ffffff;"</code></pre>
                    <pre data-prefix="12"><code>"  --color-base-200: #f2f2f2;"</code></pre>
                    <pre data-prefix="13"><code>"  --color-base-300: #e5e6e6;"</code></pre>
                    <pre data-prefix="14"><code>"  --color-base-content: #1f2937;"</code></pre>
                    <pre data-prefix="15"><code>"  --radius-selector: 0.5rem;"</code></pre>
                    <pre data-prefix="16"><code>"  --radius-field: 0.5rem;"</code></pre>
                    <pre data-prefix="17"><code>"  --radius-box: 0.5rem;"</code></pre>
                    <pre data-prefix="18"><code>"  --border: 1px;"</code></pre>
                    <pre data-prefix="19"><code>"  --depth: 1;"</code></pre>
                    <pre data-prefix="20"><code>"  --noise: 0;"</code></pre>
                    <pre data-prefix="21"><code>"}"</code></pre>
                </div>
            </section>

            // --- Section 4: CSS Variable Reference ---
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

            // --- Section 5: Tips ---
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

            // --- Section 6: Tailwind + Source Scanning ---
            <section class="space-y-4">
                <h2 class="text-2xl font-semibold">"Tailwind v4 Source Scanning"</h2>
                <p>
                    "If you use leptos-daisyui as a separate crate, Tailwind v4 might not scan its classes automatically. "
                    "Add an " <code>"@source"</code> " directive pointing to the crate source:"
                </p>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"@source \"../../crates/leptos-daisyui/src\";"</code></pre>
                    <pre data-prefix="2"><code>"@source \"../src\";  /* your app source */"</code></pre>
                </div>
                <p>"This ensures Tailwind includes all utility classes used by library components."</p>
            </section>

            // --- Quick links ---
            <section class="flex flex-wrap gap-3">
                <a href="/themes" class="btn btn-primary btn-outline">"🎨 Theme Showcase"</a>
                <a href="/themes/creator" class="btn btn-secondary btn-outline">"🔧 Theme Creator"</a>
                <a href="https://daisyui.com/docs/themes/" target="_blank" class="btn btn-ghost btn-outline">"📚 daisyUI Docs ↗"</a>
            </section>
        </div>
    }
}
