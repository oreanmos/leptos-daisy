use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RangePage() -> impl IntoView {
    let (volume, set_volume) = signal(42.0_f64);

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Range"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Slider inputs for continuous values with full size/color/state coverage."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Range size={Size::ExtraSmall} value=50.0 />
<Range size={Size::Small} value=50.0 />
<Range size={Size::Medium} value=50.0 />
<Range size={Size::Large} value=50.0 />
<Range size={Size::ExtraLarge} value=50.0 />"##
                >
                    <div class="space-y-4 max-w-xl">
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Extra Small"</span>
                            <Range size={Size::ExtraSmall} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Small"</span>
                            <Range size={Size::Small} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Medium"</span>
                            <Range size={Size::Medium} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Large"</span>
                            <Range size={Size::Large} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Extra Large"</span>
                            <Range size={Size::ExtraLarge} value=50.0 />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<Range color={Color::Primary} value=50.0 />
<Range color={Color::Secondary} value=50.0 />
<Range color={Color::Accent} value=50.0 />
<Range color={Color::Info} value=50.0 />
<Range color={Color::Success} value=50.0 />
<Range color={Color::Warning} value=50.0 />
<Range color={Color::Error} value=50.0 />"##
                >
                    <div class="space-y-4 max-w-xl">
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Primary"</span>
                            <Range color={Color::Primary} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Secondary"</span>
                            <Range color={Color::Secondary} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Accent"</span>
                            <Range color={Color::Accent} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Info"</span>
                            <Range color={Color::Info} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Success"</span>
                            <Range color={Color::Success} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Warning"</span>
                            <Range color={Color::Warning} value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Error"</span>
                            <Range color={Color::Error} value=50.0 />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="States and custom steps"
                    code=r##"<Range disabled=true value=50.0 />
<Range min=0.0 max=100.0 step=10.0 value=50.0 />
<Range min=0.0 max=10.0 step=0.5 value=5.0 />"##
                >
                    <div class="space-y-4 max-w-xl">
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Disabled"</span>
                            <Range disabled=true value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Range 0-100, step 10"</span>
                            <Range min=0.0 max=100.0 step=10.0 value=50.0 />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Range 0-10, step 0.5"</span>
                            <Range min=0.0 max=10.0 step=0.5 value=5.0 />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive example"
                    code=r##"let (volume, set_volume) = signal(42.0_f64);

view! {
    <Range
        name="volume"
        color={Color::Primary}
        min=0.0
        max=100.0
        step=1.0
        value=volume
        on:input=move |ev| {
            let next = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
            set_volume.set(next);
        }
    />
    <p class="text-sm text-base-content/70">
        {move || format!("Volume: {:.0}", volume.get())}
    </p>
}"##
                >
                    <div class="space-y-3 max-w-xl">
                        <Range
                            name="volume"
                            color={Color::Primary}
                            min=0.0
                            max=100.0
                            step=1.0
                            value=volume
                            on:input=move |ev| {
                                let next = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_volume.set(next);
                            }
                        />
                        <p class="text-sm text-base-content/70">{move || format!("Volume: {:.0}", volume.get())}</p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
