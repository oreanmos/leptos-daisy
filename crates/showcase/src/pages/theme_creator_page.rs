use leptos::prelude::*;

use crate::components::CodeBlock;

/// A single color variable entry for the theme creator.
struct ColorVar {
    label: &'static str,
    css_var: &'static str,
    default: &'static str,
}

const COLOR_VARS: &[ColorVar] = &[
    ColorVar {
        label: "Primary",
        css_var: "--color-primary",
        default: "#570df8",
    },
    ColorVar {
        label: "Primary Content",
        css_var: "--color-primary-content",
        default: "#ffffff",
    },
    ColorVar {
        label: "Secondary",
        css_var: "--color-secondary",
        default: "#f000b8",
    },
    ColorVar {
        label: "Secondary Content",
        css_var: "--color-secondary-content",
        default: "#ffffff",
    },
    ColorVar {
        label: "Accent",
        css_var: "--color-accent",
        default: "#37cdbe",
    },
    ColorVar {
        label: "Accent Content",
        css_var: "--color-accent-content",
        default: "#163835",
    },
    ColorVar {
        label: "Neutral",
        css_var: "--color-neutral",
        default: "#3d4451",
    },
    ColorVar {
        label: "Neutral Content",
        css_var: "--color-neutral-content",
        default: "#ffffff",
    },
    ColorVar {
        label: "Base 100",
        css_var: "--color-base-100",
        default: "#ffffff",
    },
    ColorVar {
        label: "Base 200",
        css_var: "--color-base-200",
        default: "#f2f2f2",
    },
    ColorVar {
        label: "Base 300",
        css_var: "--color-base-300",
        default: "#e5e6e6",
    },
    ColorVar {
        label: "Base Content",
        css_var: "--color-base-content",
        default: "#1f2937",
    },
    ColorVar {
        label: "Info",
        css_var: "--color-info",
        default: "#3abff8",
    },
    ColorVar {
        label: "Info Content",
        css_var: "--color-info-content",
        default: "#002b3d",
    },
    ColorVar {
        label: "Success",
        css_var: "--color-success",
        default: "#36d399",
    },
    ColorVar {
        label: "Success Content",
        css_var: "--color-success-content",
        default: "#003320",
    },
    ColorVar {
        label: "Warning",
        css_var: "--color-warning",
        default: "#fbbd23",
    },
    ColorVar {
        label: "Warning Content",
        css_var: "--color-warning-content",
        default: "#382800",
    },
    ColorVar {
        label: "Error",
        css_var: "--color-error",
        default: "#f87272",
    },
    ColorVar {
        label: "Error Content",
        css_var: "--color-error-content",
        default: "#470000",
    },
];

#[component]
pub fn ThemeCreatorPage() -> impl IntoView {
    // Create reactive signals for each color
    let color_signals: Vec<(
        &'static str,
        &'static str,
        ReadSignal<String>,
        WriteSignal<String>,
    )> = COLOR_VARS
        .iter()
        .map(|cv| {
            let (read, write) = signal(cv.default.to_string());
            (cv.label, cv.css_var, read, write)
        })
        .collect();

    let (theme_name, set_theme_name) = signal("my-custom-theme".to_string());
    let (border_radius, set_border_radius) = signal("0.5".to_string());
    let (border_width, set_border_width) = signal("1".to_string());
    let (depth, set_depth) = signal("1".to_string());
    let (noise, set_noise) = signal("0".to_string());

    // Build the live CSS custom properties string for the preview
    let color_signals_for_style = color_signals.clone();
    let preview_style = move || {
        let mut style = String::new();
        for (_, css_var, read, _) in &color_signals_for_style {
            let val = read.get();
            style.push_str(&format!("{}: {}; ", css_var, val));
        }
        style.push_str(&format!("--radius-selector: {}rem; ", border_radius.get()));
        style.push_str(&format!("--radius-field: {}rem; ", border_radius.get()));
        style.push_str(&format!("--radius-box: {}rem; ", border_radius.get()));
        style.push_str(&format!("--border: {}px; ", border_width.get()));
        style.push_str(&format!("--depth: {}; ", depth.get()));
        style.push_str(&format!("--noise: {}; ", noise.get()));
        style
    };

    // Generate CSS output
    let color_signals_for_output = color_signals.clone();
    let css_output = move || {
        let name = theme_name.get();
        let mut lines = vec![format!("@plugin \"daisyui/theme\" {{")];
        lines.push(format!("  name: \"{}\";", name));
        for (label, css_var, read, _) in &color_signals_for_output {
            let val = read.get();
            lines.push(format!("  {}: {};  /* {} */", css_var, val, label));
        }
        lines.push(format!("  --radius-selector: {}rem;", border_radius.get()));
        lines.push(format!("  --radius-field: {}rem;", border_radius.get()));
        lines.push(format!("  --radius-box: {}rem;", border_radius.get()));
        lines.push(format!("  --border: {}px;", border_width.get()));
        lines.push(format!("  --depth: {};", depth.get()));
        lines.push(format!("  --noise: {};", noise.get()));
        lines.push("}".to_string());
        lines.join("\n")
    };

    let (copied, _set_copied) = signal(false);

    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"🎨 Theme Creator"</h1>
            <p class="text-base-content/70">
                "Design your own daisyUI theme by customizing colors and design tokens. "
                "The generated CSS can be added to your stylesheet."
            </p>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                // --- Controls Column ---
                <div class="space-y-6">
                    // Theme name
                    <div class="form-control">
                        <label class="label"><span class="label-text font-semibold">"Theme Name"</span></label>
                        <input
                            type="text"
                            class="input input-bordered"
                            prop:value=move || theme_name.get()
                            on:input=move |ev| set_theme_name.set(event_target_value(&ev))
                        />
                    </div>

                    // Color pickers
                    <div class="space-y-1">
                        <h3 class="text-lg font-semibold">"Colors"</h3>
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            {color_signals.iter().map(|(label, _css_var, read, write)| {
                                let label = *label;
                                let read = *read;
                                let write = *write;
                                view! {
                                    <div class="form-control">
                                        <label class="label py-0.5">
                                            <span class="label-text text-xs">{label}</span>
                                        </label>
                                        <div class="flex items-center gap-2">
                                            <input
                                                type="color"
                                                class="w-8 h-8 rounded cursor-pointer border-0"
                                                prop:value=move || read.get()
                                                on:input=move |ev| write.set(event_target_value(&ev))
                                            />
                                            <input
                                                type="text"
                                                class="input input-bordered input-xs flex-1 font-mono"
                                                prop:value=move || read.get()
                                                on:input=move |ev| write.set(event_target_value(&ev))
                                            />
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    // Design tokens
                    <div class="space-y-3">
                        <h3 class="text-lg font-semibold">"Design Tokens"</h3>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text text-sm">"Border Radius (rem)"</span>
                                <span class="label-text-alt">{move || border_radius.get()}</span>
                            </label>
                            <input
                                type="range"
                                min="0" max="2" step="0.1"
                                class="range range-primary range-sm"
                                prop:value=move || border_radius.get()
                                on:input=move |ev| set_border_radius.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text text-sm">"Border Width (px)"</span>
                                <span class="label-text-alt">{move || border_width.get()}</span>
                            </label>
                            <input
                                type="range"
                                min="0" max="4" step="1"
                                class="range range-primary range-sm"
                                prop:value=move || border_width.get()
                                on:input=move |ev| set_border_width.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text text-sm">"Depth (shadow)"</span>
                                <span class="label-text-alt">{move || depth.get()}</span>
                            </label>
                            <input
                                type="range"
                                min="0" max="2" step="1"
                                class="range range-primary range-sm"
                                prop:value=move || depth.get()
                                on:input=move |ev| set_depth.set(event_target_value(&ev))
                            />
                        </div>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text text-sm">"Noise (texture)"</span>
                                <span class="label-text-alt">{move || noise.get()}</span>
                            </label>
                            <input
                                type="range"
                                min="0" max="1" step="1"
                                class="range range-primary range-sm"
                                prop:value=move || noise.get()
                                on:input=move |ev| set_noise.set(event_target_value(&ev))
                            />
                        </div>
                    </div>
                </div>

                // --- Preview + Output Column ---
                <div class="space-y-6">
                    // Live preview
                    <div>
                        <h3 class="text-lg font-semibold mb-3">"Live Preview"</h3>
                        <div
                            class="rounded-box border border-base-300 bg-base-100 p-4 space-y-4"
                            style=preview_style
                        >
                            <h4 class="text-lg font-bold text-base-content">"Preview Card"</h4>
                            // Color swatches
                            <div class="flex gap-1.5">
                                <div class="bg-primary rounded-btn w-8 h-8" title="primary"></div>
                                <div class="bg-secondary rounded-btn w-8 h-8" title="secondary"></div>
                                <div class="bg-accent rounded-btn w-8 h-8" title="accent"></div>
                                <div class="bg-neutral rounded-btn w-8 h-8" title="neutral"></div>
                            </div>
                            <div class="flex gap-1.5">
                                <div class="bg-base-100 border border-base-300 rounded w-8 h-8"></div>
                                <div class="bg-base-200 rounded w-8 h-8"></div>
                                <div class="bg-base-300 rounded w-8 h-8"></div>
                            </div>
                            // Buttons
                            <div class="flex flex-wrap gap-2">
                                <button class="btn btn-primary btn-sm">"Primary"</button>
                                <button class="btn btn-secondary btn-sm">"Secondary"</button>
                                <button class="btn btn-accent btn-sm">"Accent"</button>
                                <button class="btn btn-ghost btn-sm">"Ghost"</button>
                            </div>
                            // Form elements
                            <div class="flex flex-wrap items-center gap-3">
                                <input type="checkbox" class="checkbox checkbox-primary" checked />
                                <input type="checkbox" class="toggle toggle-secondary" checked />
                                <div class="badge badge-primary">"badge"</div>
                                <div class="badge badge-secondary badge-outline">"outline"</div>
                            </div>
                            // Status colors
                            <div class="flex gap-1.5">
                                <div class="bg-info rounded w-6 h-6"></div>
                                <div class="bg-success rounded w-6 h-6"></div>
                                <div class="bg-warning rounded w-6 h-6"></div>
                                <div class="bg-error rounded w-6 h-6"></div>
                            </div>
                            <progress class="progress progress-primary w-full" value="70" max="100"></progress>
                            // Alerts
                            <div class="alert alert-info py-2">
                                <span class="text-xs">"Info message"</span>
                            </div>
                            <p class="text-sm text-base-content/70">"The quick brown fox jumps over the lazy dog."</p>
                        </div>
                    </div>

                    // Generated CSS
                    <div>
                        <div class="flex items-center justify-between mb-3">
                            <h3 class="text-lg font-semibold">"Generated CSS"</h3>
                            <button
                                class="btn btn-sm btn-outline"
                                on:click={
                                    let _css_output_clone = css_output.clone();
                                    move |_| {
                                        #[cfg(any(feature = "csr", feature = "hydrate"))]
                                        {
                                            let text = _css_output_clone();
                                            let _ = leptos::prelude::window().navigator().clipboard().write_text(&text);
                                            _set_copied.set(true);
                                            set_timeout(move || _set_copied.set(false), std::time::Duration::from_secs(2));
                                        }
                                    }
                                }
                            >
                                {move || if copied.get() { "✓ Copied!" } else { "📋 Copy" }}
                            </button>
                        </div>
                        {move || {
                            let code = css_output();
                            view! {
                                <CodeBlock code=code language="css" class="max-h-96 overflow-auto" />
                            }
                        }}
                    </div>

                    // Instructions
                    <div class="alert">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-info shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <p class="font-bold text-sm">"How to use"</p>
                            <p class="text-xs">"Copy the CSS above and paste it into your main stylesheet after your daisyUI plugin import. "
                                "Then use " <code>"data-theme=\"your-theme-name\""</code> " on your HTML element."</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
