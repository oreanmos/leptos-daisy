use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let (marketing_opt_in, set_marketing_opt_in) = signal(false);
    let marketing_opt_in_signal = Signal::derive(move || marketing_opt_in.get());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Checkbox"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Boolean form controls with complete color/size/state coverage and label patterns."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<Checkbox />
<Checkbox color={Color::Primary} checked=true />
<Checkbox color={Color::Secondary} checked=true />
<Checkbox color={Color::Accent} checked=true />
<Checkbox color={Color::Success} checked=true />
<Checkbox color={Color::Warning} checked=true />
<Checkbox color={Color::Error} checked=true />"##
                >
                    <div class="flex flex-wrap gap-4">
                        <Checkbox />
                        <Checkbox color={Color::Primary} checked=true />
                        <Checkbox color={Color::Secondary} checked=true />
                        <Checkbox color={Color::Accent} checked=true />
                        <Checkbox color={Color::Success} checked=true />
                        <Checkbox color={Color::Warning} checked=true />
                        <Checkbox color={Color::Error} checked=true />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Checkbox size={Size::ExtraSmall} checked=true />
<Checkbox size={Size::Small} checked=true />
<Checkbox size={Size::Medium} checked=true />
<Checkbox size={Size::Large} checked=true />
<Checkbox size={Size::ExtraLarge} checked=true />"##
                >
                    <div class="flex flex-wrap items-center gap-4">
                        <Checkbox size={Size::ExtraSmall} checked=true />
                        <Checkbox size={Size::Small} checked=true />
                        <Checkbox size={Size::Medium} checked=true />
                        <Checkbox size={Size::Large} checked=true />
                        <Checkbox size={Size::ExtraLarge} checked=true />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With labels"
                    code=r##"<label class="label cursor-pointer justify-start gap-3">
    <Checkbox name="terms" />
    <span class="label-text">"I agree to the terms of service"</span>
</label>

<label class="label cursor-pointer justify-start gap-3">
    <Checkbox name="updates" checked=true color={Color::Primary} />
    <span class="label-text">"Send me release updates"</span>
</label>

<label class="label cursor-not-allowed justify-start gap-3 opacity-70">
    <Checkbox disabled=true checked=true />
    <span class="label-text">"Disabled and checked"</span>
</label>"##
                >
                    <div class="space-y-3 max-w-md">
                        <label class="label cursor-pointer justify-start gap-3">
                            <Checkbox name="terms" />
                            <span class="label-text">"I agree to the terms of service"</span>
                        </label>
                        <label class="label cursor-pointer justify-start gap-3">
                            <Checkbox name="updates" checked=true color={Color::Primary} />
                            <span class="label-text">"Send me release updates"</span>
                        </label>
                        <label class="label cursor-not-allowed justify-start gap-3 opacity-70">
                            <Checkbox disabled=true checked=true />
                            <span class="label-text">"Disabled and checked"</span>
                        </label>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive state"
                    code=r##"let (marketing_opt_in, set_marketing_opt_in) = signal(false);
let marketing_opt_in_signal = Signal::derive(move || marketing_opt_in.get());

view! {
    <label class="label cursor-pointer justify-start gap-3">
        <Checkbox
            name="marketing"
            checked=marketing_opt_in_signal
            on:change=move |ev| set_marketing_opt_in.set(event_target_checked(&ev))
        />
        <span class="label-text">"Receive marketing emails"</span>
    </label>
    <p class="text-sm text-base-content/70">
        {move || format!("opt in: {}", marketing_opt_in.get())}
    </p>
}"##
                >
                    <div class="space-y-3 max-w-md">
                        <label class="label cursor-pointer justify-start gap-3">
                            <Checkbox
                                name="marketing"
                                checked=marketing_opt_in_signal
                                on:change=move |ev| set_marketing_opt_in.set(event_target_checked(&ev))
                            />
                            <span class="label-text">"Receive marketing emails"</span>
                        </label>
                        <p class="text-sm text-base-content/70">
                            {move || format!("opt in: {}", marketing_opt_in.get())}
                        </p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
