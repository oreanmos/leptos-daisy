use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AestheticGalleryPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Aesthetic Gallery"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Compare how the same content looks under each aesthetic preset."
                </p>
            </header>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {Aesthetic::all().iter().filter(|a| **a != Aesthetic::Auto).map(|a| {
                    let preset = a.preset();
                    let theme_str = preset.daisy_theme.map(|t| t.as_str()).unwrap_or("light");
                    view! {
                        <div data-aesthetic=a.as_str() data-theme=theme_str class="rounded-box border border-base-300 overflow-hidden">
                            <div class="bg-base-200 px-4 py-2 border-b border-base-300">
                                <span class="font-bold text-sm">{preset.label}</span>
                            </div>
                            <div class="bg-base-100 p-4 space-y-3">
                                <h3 class="text-lg font-bold" style="font-family: var(--font-heading)">"Heading Text"</h3>
                                <p style="font-family: var(--font-body)">"Body text demonstrating the aesthetic's typography choices."</p>
                                <div class="flex gap-2">
                                    <button class="btn btn-primary btn-sm">"Primary"</button>
                                    <button class="btn btn-sm">"Default"</button>
                                </div>
                                <div class="aesthetic-card bg-base-200 p-3">
                                    <p class="text-sm">"Card with aesthetic styling"</p>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
