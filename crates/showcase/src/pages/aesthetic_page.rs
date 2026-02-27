use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn AestheticPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Aesthetic System"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "The 2-layer theme system: DaisyUI handles colors (data-theme), "
                    "aesthetics handle typography, spacing, shadows, and radii (data-aesthetic)."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Available Aesthetics"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {Aesthetic::all().iter().map(|a| {
                        let preset = a.preset();
                        view! {
                            <div class="card bg-base-200 shadow-sm">
                                <div class="card-body p-4">
                                    <h3 class="card-title text-sm">{preset.label}</h3>
                                    <p class="text-xs text-base-content/60">{preset.description}</p>
                                    <div class="flex gap-2 mt-2">
                                        <span class="badge badge-sm badge-ghost">{format!("data-aesthetic=\"{}\"", preset.id)}</span>
                                        {preset.daisy_theme.map(|t| view! {
                                            <span class="badge badge-sm badge-ghost">{format!("data-theme=\"{}\"", t.as_str())}</span>
                                        })}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"CSS Custom Properties"</h2>
                <p class="text-base-content/70">"Each aesthetic sets these 13 CSS custom properties:"</p>
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>"Property"</th>
                                <th>"Purpose"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr><td><code>"--font-heading"</code></td><td>"Heading font family"</td></tr>
                            <tr><td><code>"--font-body"</code></td><td>"Body text font family"</td></tr>
                            <tr><td><code>"--font-mono"</code></td><td>"Monospace font family"</td></tr>
                            <tr><td><code>"--radius-card"</code></td><td>"Card border radius"</td></tr>
                            <tr><td><code>"--radius-btn"</code></td><td>"Button border radius"</td></tr>
                            <tr><td><code>"--radius-input"</code></td><td>"Input border radius"</td></tr>
                            <tr><td><code>"--shadow-card"</code></td><td>"Card shadow"</td></tr>
                            <tr><td><code>"--shadow-card-hover"</code></td><td>"Card hover shadow"</td></tr>
                            <tr><td><code>"--spacing-page-x"</code></td><td>"Horizontal page padding"</td></tr>
                            <tr><td><code>"--spacing-page-y"</code></td><td>"Vertical page padding"</td></tr>
                            <tr><td><code>"--spacing-section"</code></td><td>"Section gap spacing"</td></tr>
                            <tr><td><code>"--border-card"</code></td><td>"Card border"</td></tr>
                            <tr><td><code>"--transition-card"</code></td><td>"Card transition"</td></tr>
                        </tbody>
                    </table>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Utility Classes"</h2>
                <ComponentPreview
                    title="aesthetic-card"
                    code=r#"<div class="aesthetic-card bg-base-100 p-4">
    "Card with aesthetic-aware styling"
</div>"#
                >
                    <div class="flex gap-4">
                        <div class="aesthetic-card bg-base-100 p-4">"aesthetic-card"</div>
                        <div class="aesthetic-page bg-base-200">"aesthetic-page"</div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
