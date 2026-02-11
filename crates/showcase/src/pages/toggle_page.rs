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
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <Toggle checked=true />
                    <Toggle color={Color::Primary} checked=true />
                    <Toggle color={Color::Secondary} checked=true />
                    <Toggle color={Color::Accent} checked=true />
                    <Toggle color={Color::Success} checked=true />
                    <Toggle color={Color::Warning} checked=true />
                    <Toggle color={Color::Error} checked=true />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-4">
                    <Toggle size={Size::ExtraSmall} checked=true />
                    <Toggle size={Size::Small} checked=true />
                    <Toggle size={Size::Medium} checked=true />
                    <Toggle size={Size::Large} checked=true />
                    <Toggle size={Size::ExtraLarge} checked=true />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"States"</h2>
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
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Reactive setting"</h2>
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
            </section>
        </div>
    }
}
