use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RadioPage() -> impl IntoView {
    let (density, set_density) = signal("comfortable".to_string());
    let density_compact = Signal::derive(move || density.get() == "compact");
    let density_comfortable = Signal::derive(move || density.get() == "comfortable");
    let density_spacious = Signal::derive(move || density.get() == "spacious");

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Radio"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Mutually exclusive options with daisyUI color, size, and form composition patterns."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <Radio name="color-demo" value="default" checked=true />
                    <Radio name="color-demo" value="primary" color={Color::Primary} />
                    <Radio name="color-demo" value="secondary" color={Color::Secondary} />
                    <Radio name="color-demo" value="accent" color={Color::Accent} />
                    <Radio name="color-demo" value="success" color={Color::Success} />
                    <Radio name="color-demo" value="warning" color={Color::Warning} />
                    <Radio name="color-demo" value="error" color={Color::Error} />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-4">
                    <Radio name="size-demo" value="xs" size={Size::ExtraSmall} checked=true />
                    <Radio name="size-demo" value="sm" size={Size::Small} />
                    <Radio name="size-demo" value="md" size={Size::Medium} />
                    <Radio name="size-demo" value="lg" size={Size::Large} />
                    <Radio name="size-demo" value="xl" size={Size::ExtraLarge} />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Labeled group"</h2>
                <Fieldset legend="Default density">
                    <label class="label cursor-pointer justify-start gap-3">
                        <Radio name="density-static" value="compact" />
                        <span class="label-text">"Compact"</span>
                    </label>
                    <label class="label cursor-pointer justify-start gap-3">
                        <Radio name="density-static" value="comfortable" checked=true />
                        <span class="label-text">"Comfortable"</span>
                    </label>
                    <label class="label cursor-not-allowed justify-start gap-3 opacity-70">
                        <Radio name="density-static" value="spacious" disabled=true />
                        <span class="label-text">"Spacious (disabled)"</span>
                    </label>
                </Fieldset>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Reactive selection"</h2>
                <Fieldset legend="Density preference">
                    <label class="label cursor-pointer justify-start gap-3">
                        <Radio
                            name="density-reactive"
                            value="compact"
                            checked=density_compact
                            on:change=move |_| set_density.set("compact".to_string())
                        />
                        <span class="label-text">"Compact"</span>
                    </label>
                    <label class="label cursor-pointer justify-start gap-3">
                        <Radio
                            name="density-reactive"
                            value="comfortable"
                            checked=density_comfortable
                            on:change=move |_| set_density.set("comfortable".to_string())
                        />
                        <span class="label-text">"Comfortable"</span>
                    </label>
                    <label class="label cursor-pointer justify-start gap-3">
                        <Radio
                            name="density-reactive"
                            value="spacious"
                            checked=density_spacious
                            on:change=move |_| set_density.set("spacious".to_string())
                        />
                        <span class="label-text">"Spacious"</span>
                    </label>
                    <p class="text-sm text-base-content/70">
                        {move || format!("selected: {}", density.get())}
                    </p>
                </Fieldset>
            </section>
        </div>
    }
}
