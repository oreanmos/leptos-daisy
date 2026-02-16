use leptos::prelude::*;

/// All 35 daisyUI built-in themes + custom themes included in this repo.
const ALL_THEMES: &[&str] = &[
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
    "dim",
    "nord",
    "sunset",
    "caramellatte",
    "abyss",
    "silk",
    "terminal",
];

#[component]
pub fn ThemeShowcasePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"🎨 Theme Showcase"</h1>
            <p class="text-base-content/70">
                "Preview all 35 built-in daisyUI themes. Each card shows a live preview "
                "plus the custom "
                <code class="badge badge-ghost badge-sm">"terminal"</code>
                " theme. Each card shows a live preview "
                "of the theme colors and component styles using "
                <code class="badge badge-ghost badge-sm">"data-theme"</code>
                " scoping."
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                {ALL_THEMES.iter().map(|theme| {
                    let theme = *theme;
                    view! {
                        <div
                            data-theme={theme}
                            class="rounded-box border border-base-300 bg-base-100 shadow-md overflow-hidden"
                        >
                            // Theme name header
                            <div class="bg-base-200 px-4 py-2 border-b border-base-300 flex items-center justify-between">
                                <span class="font-bold text-base-content">{theme}</span>
                                <button
                                    class="btn btn-xs btn-ghost"
                                    on:click=move |_| {
                                        #[cfg(feature = "csr")]
                                        {
                                            if let Some(doc) = leptos::prelude::document().document_element() {
                                                let _ = doc.set_attribute("data-theme", theme);
                                            }
                                            if let Ok(Some(storage)) = leptos::prelude::window().local_storage() {
                                                let _ = storage.set_item("daisy-theme", theme);
                                            }
                                        }
                                    }
                                >
                                    "Apply"
                                </button>
                            </div>

                            <div class="p-4 space-y-3">
                                // Color swatches
                                <div class="flex gap-1">
                                    <div class="bg-primary rounded-btn w-6 h-6" title="primary"></div>
                                    <div class="bg-secondary rounded-btn w-6 h-6" title="secondary"></div>
                                    <div class="bg-accent rounded-btn w-6 h-6" title="accent"></div>
                                    <div class="bg-neutral rounded-btn w-6 h-6" title="neutral"></div>
                                </div>

                                // Base colors
                                <div class="flex gap-1">
                                    <div class="bg-base-100 border border-base-300 rounded w-6 h-6" title="base-100"></div>
                                    <div class="bg-base-200 border border-base-300 rounded w-6 h-6" title="base-200"></div>
                                    <div class="bg-base-300 border border-base-300 rounded w-6 h-6" title="base-300"></div>
                                </div>

                                // Status colors
                                <div class="flex gap-1">
                                    <div class="bg-info rounded w-4 h-4" title="info"></div>
                                    <div class="bg-success rounded w-4 h-4" title="success"></div>
                                    <div class="bg-warning rounded w-4 h-4" title="warning"></div>
                                    <div class="bg-error rounded w-4 h-4" title="error"></div>
                                </div>

                                // Sample components
                                <div class="flex flex-wrap gap-2">
                                    <button class="btn btn-primary btn-xs">"Primary"</button>
                                    <button class="btn btn-secondary btn-xs">"Secondary"</button>
                                    <button class="btn btn-accent btn-xs">"Accent"</button>
                                </div>

                                // Sample form elements
                                <div class="flex items-center gap-2">
                                    <input type="checkbox" class="checkbox checkbox-primary checkbox-xs" checked />
                                    <input type="checkbox" class="toggle toggle-primary toggle-xs" checked />
                                    <div class="badge badge-primary badge-sm">"badge"</div>
                                </div>

                                // Progress bar
                                <progress class="progress progress-primary w-full" value="65" max="100"></progress>

                                // Text sample
                                <p class="text-xs text-base-content/70">"The quick brown fox jumps over the lazy dog."</p>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
