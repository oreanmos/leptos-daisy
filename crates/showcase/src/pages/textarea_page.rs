use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TextareaPage() -> impl IntoView {
    let (bio, set_bio) = signal(String::new());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Textarea"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Multi-line inputs with full variant coverage and common composition patterns."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-col gap-3 max-w-lg">
                    <Textarea placeholder="Default textarea" rows=3 />
                    <Textarea color={Color::Neutral} placeholder="Neutral" rows=3 />
                    <Textarea color={Color::Primary} placeholder="Primary" rows=3 />
                    <Textarea color={Color::Secondary} placeholder="Secondary" rows=3 />
                    <Textarea color={Color::Accent} placeholder="Accent" rows=3 />
                    <Textarea color={Color::Info} placeholder="Info" rows=3 />
                    <Textarea color={Color::Success} placeholder="Success" rows=3 />
                    <Textarea color={Color::Warning} placeholder="Warning" rows=3 />
                    <Textarea color={Color::Error} placeholder="Error" rows=3 />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex flex-col gap-3 max-w-lg">
                    <Textarea size={Size::ExtraSmall} placeholder="Extra Small" rows=2 />
                    <Textarea size={Size::Small} placeholder="Small" rows=2 />
                    <Textarea size={Size::Medium} placeholder="Medium" rows=3 />
                    <Textarea size={Size::Large} placeholder="Large" rows=4 />
                    <Textarea size={Size::ExtraLarge} placeholder="Extra Large" rows=5 />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Styles and states"</h2>
                <div class="flex flex-col gap-3 max-w-lg">
                    <Textarea variant={Variant::Outline} placeholder="Bordered / outline style" rows=3 />
                    <Textarea variant={Variant::Ghost} placeholder="Ghost style" rows=3 />
                    <Textarea disabled=true placeholder="Disabled textarea" rows=3 />
                    <Textarea required=true placeholder="Required textarea" rows=3 />
                    <Textarea readonly=true value="Readonly note content" rows=3 />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"With fieldset and helper text"</h2>
                <div class="max-w-lg">
                    <Fieldset legend="Project summary">
                        <Textarea
                            name="summary"
                            rows=4
                            placeholder="Write a short summary of your project"
                        />
                        <p class="label">"Keep it under 500 characters."</p>
                    </Fieldset>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Reactive binding"</h2>
                <div class="max-w-lg space-y-3">
                    <Textarea
                        rows=4
                        placeholder="Type your bio"
                        prop:value=move || bio.get()
                        on:input=move |ev| set_bio.set(event_target_value(&ev))
                    />
                    <p class="text-sm text-base-content/70">
                        {move || format!("{} characters", bio.get().chars().count())}
                    </p>
                </div>
            </section>
        </div>
    }
}
