use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TogglePage() -> impl IntoView {
    let (dark_mode, set_dark_mode) = signal(false);
    let dark_mode_signal = Signal::derive(move || dark_mode.get());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Toggle"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Switch-style checkboxes for settings and preferences."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "toggle", type_label: "base", description: "Base toggle switch component" },
                    ClassEntry { name: "toggle-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "toggle-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "toggle-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "toggle-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "toggle-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "toggle-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "toggle-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "toggle-lg", type_label: "size", description: "Large size" },
                    ClassEntry { name: "toggle-md", type_label: "size", description: "Medium size" },
                    ClassEntry { name: "toggle-sm", type_label: "size", description: "Small size" },
                    ClassEntry { name: "toggle-xs", type_label: "size", description: "Extra small size" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<Toggle checked=true />
<Toggle color={Color::Primary} checked=true />
<Toggle color={Color::Secondary} checked=true />
<Toggle color={Color::Accent} checked=true />
<Toggle color={Color::Success} checked=true />
<Toggle color={Color::Warning} checked=true />
<Toggle color={Color::Error} checked=true />"##
                >
                    <div class="flex flex-wrap gap-4">
                        <Toggle checked=true />
                        <Toggle color={Color::Primary} checked=true />
                        <Toggle color={Color::Secondary} checked=true />
                        <Toggle color={Color::Accent} checked=true />
                        <Toggle color={Color::Success} checked=true />
                        <Toggle color={Color::Warning} checked=true />
                        <Toggle color={Color::Error} checked=true />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Toggle size={Size::ExtraSmall} checked=true />
<Toggle size={Size::Small} checked=true />
<Toggle size={Size::Medium} checked=true />
<Toggle size={Size::Large} checked=true />
<Toggle size={Size::ExtraLarge} checked=true />"##
                >
                    <div class="flex items-center gap-4">
                        <Toggle size={Size::ExtraSmall} checked=true />
                        <Toggle size={Size::Small} checked=true />
                        <Toggle size={Size::Medium} checked=true />
                        <Toggle size={Size::Large} checked=true />
                        <Toggle size={Size::ExtraLarge} checked=true />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="States"
                    code=r##"<label class="label cursor-pointer justify-start gap-3">
    <Toggle checked=true />
    <span class="label-text">"Enabled"</span>
</label>

<label class="label cursor-not-allowed justify-start gap-3 opacity-70">
    <Toggle disabled=true checked=true />
    <span class="label-text">"Disabled (on)"</span>
</label>

<label class="label cursor-not-allowed justify-start gap-3 opacity-70">
    <Toggle disabled=true />
    <span class="label-text">"Disabled (off)"</span>
</label>"##
                >
                    <div class="flex flex-col gap-3 max-w-sm">
                        <label class="label cursor-pointer justify-start gap-3">
                            <Toggle checked=true />
                            <span class="label-text">"Enabled"</span>
                        </label>
                        <label class="label cursor-not-allowed justify-start gap-3 opacity-70">
                            <Toggle disabled=true checked=true />
                            <span class="label-text">"Disabled (on)"</span>
                        </label>
                        <label class="label cursor-not-allowed justify-start gap-3 opacity-70">
                            <Toggle disabled=true />
                            <span class="label-text">"Disabled (off)"</span>
                        </label>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive setting"
                    code=r##"let (dark_mode, set_dark_mode) = signal(false);
let dark_mode_signal = Signal::derive(move || dark_mode.get());

view! {
    <label class="label cursor-pointer justify-start gap-3">
        <Toggle
            color={Color::Primary}
            checked=dark_mode_signal
            on:change=move |ev| set_dark_mode.set(event_target_checked(&ev))
        />
        <span class="label-text">"Use dark mode"</span>
    </label>
    <p class="text-sm text-base-content/70">
        {move || format!("dark mode: {}", dark_mode.get())}
    </p>
}"##
                >
                    <div class="max-w-sm space-y-3">
                        <label class="label cursor-pointer justify-start gap-3">
                            <Toggle
                                color={Color::Primary}
                                checked=dark_mode_signal
                                on:change=move |ev| set_dark_mode.set(event_target_checked(&ev))
                            />
                            <span class="label-text">"Use dark mode"</span>
                        </label>
                        <p class="text-sm text-base-content/70">
                            {move || format!("dark mode: {}", dark_mode.get())}
                        </p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
