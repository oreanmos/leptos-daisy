use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::CodeBlock;

#[component]
pub fn HomePage() -> impl IntoView {
    let getting_started_code = r#"// Cargo.toml
[dependencies]
leptos-daisyui = "0.1"

// src/main.rs
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

fn main() {
    mount_to_body(|| view! {
        <Button color=Color::Primary size=Size::Lg>
            "Hello DaisyUI!"
        </Button>
    });
}"#;

    view! {
        <div>
            // Hero Section
            <div class="hero min-h-[60vh] bg-base-200 rounded-box">
                <div class="hero-content text-center">
                    <div class="max-w-2xl">
                        <h1 class="text-5xl md:text-6xl font-bold tracking-tight">"leptos-daisyui"</h1>
                        <p class="text-xl md:text-2xl font-semibold mt-4 text-base-content/80">
                            "65+ type-safe Leptos components wrapping DaisyUI"
                        </p>
                        <p class="py-6 text-base-content/70 max-w-lg mx-auto">
                            "A comprehensive Rust component library that brings the full power of DaisyUI to Leptos. "
                            "Type-checked props, compile-time safety, and seamless integration with the Leptos reactive framework."
                        </p>
                        <div class="flex flex-wrap justify-center gap-3">
                            <A href="/button" attr:class="btn btn-primary btn-lg">"Browse Components"</A>
                            <A href="/playground" attr:class="btn btn-secondary btn-lg">"Playground"</A>
                            <a href="#" class="btn btn-ghost btn-outline btn-lg">"GitHub"</a>
                        </div>
                    </div>
                </div>
            </div>

            // Quick Stats
            <div class="flex justify-center my-12">
                <div class="stats stats-vertical md:stats-horizontal shadow bg-base-100">
                    <div class="stat">
                        <div class="stat-value text-primary">"65+"</div>
                        <div class="stat-title">"Components"</div>
                    </div>
                    <div class="stat">
                        <div class="stat-value text-secondary">"35"</div>
                        <div class="stat-title">"Themes"</div>
                    </div>
                    <div class="stat">
                        <div class="stat-value text-accent">"3"</div>
                        <div class="stat-title">"Render Modes"</div>
                    </div>
                    <div class="stat">
                        <div class="stat-value text-info">"0"</div>
                        <div class="stat-title">"Custom CSS Files"</div>
                    </div>
                </div>
            </div>

            // Feature Highlights
            <div class="my-16">
                <h2 class="text-3xl font-bold text-center mb-8">"Why leptos-daisyui?"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="card bg-base-100 shadow-md border border-primary/20">
                        <div class="card-body">
                            <h3 class="card-title text-primary">"Type-Safe"</h3>
                            <p class="text-base-content/70">
                                "Every component prop is type-checked at compile time with Rust's type system"
                            </p>
                        </div>
                    </div>
                    <div class="card bg-base-100 shadow-md border border-secondary/20">
                        <div class="card-body">
                            <h3 class="card-title text-secondary">"SSR / CSR / Hydrate"</h3>
                            <p class="text-base-content/70">
                                "Three rendering modes supported out of the box. Build for any deployment target"
                            </p>
                        </div>
                    </div>
                    <div class="card bg-base-100 shadow-md border border-accent/20">
                        <div class="card-body">
                            <h3 class="card-title text-accent">"35 Themes"</h3>
                            <p class="text-base-content/70">
                                "All DaisyUI themes included. Switch themes with a single data attribute"
                            </p>
                        </div>
                    </div>
                    <div class="card bg-base-100 shadow-md border border-info/20">
                        <div class="card-body">
                            <h3 class="card-title text-info">"Zero Custom CSS"</h3>
                            <p class="text-base-content/70">
                                "Built entirely on Tailwind CSS and DaisyUI utility classes. No custom stylesheets needed"
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Component Categories
            <div class="my-16">
                <h2 class="text-3xl font-bold text-center mb-8">"Component Categories"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <A href="/button" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Actions"</h3>
                            <div class="badge badge-primary">"4"</div>
                        </div>
                    </A>
                    <A href="/alert" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Data Display"</h3>
                            <div class="badge badge-secondary">"18"</div>
                        </div>
                    </A>
                    <A href="/checkbox" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Data Input"</h3>
                            <div class="badge badge-accent">"12"</div>
                        </div>
                    </A>
                    <A href="/loading" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Feedback"</h3>
                            <div class="badge badge-info">"3"</div>
                        </div>
                    </A>
                    <A href="/artboard" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Layout"</h3>
                            <div class="badge badge-success">"13"</div>
                        </div>
                    </A>
                    <A href="/mockup-browser" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Mockups"</h3>
                            <div class="badge badge-warning">"4"</div>
                        </div>
                    </A>
                    <A href="/breadcrumbs" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Navigation"</h3>
                            <div class="badge badge-error">"9"</div>
                        </div>
                    </A>
                    <A href="/backdrop" attr:class="card bg-base-100 shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <div class="card-body items-center text-center p-4">
                            <h3 class="card-title text-sm">"Overlay"</h3>
                            <div class="badge badge-neutral">"2"</div>
                        </div>
                    </A>
                </div>
            </div>

            // Getting Started
            <div class="my-16">
                <h2 class="text-3xl font-bold text-center mb-8">"Get Started"</h2>
                <div class="max-w-2xl mx-auto">
                    <CodeBlock code=getting_started_code.to_string() />
                </div>
            </div>
        </div>
    }
}
