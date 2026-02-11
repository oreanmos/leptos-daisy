use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn InputPage() -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Input"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Single-line text fields with complete daisyUI color, size, and state coverage."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-col gap-3 max-w-md">
                    <Input placeholder="Default input" />
                    <Input color={Color::Neutral} placeholder="Neutral" />
                    <Input color={Color::Primary} placeholder="Primary" />
                    <Input color={Color::Secondary} placeholder="Secondary" />
                    <Input color={Color::Accent} placeholder="Accent" />
                    <Input color={Color::Info} placeholder="Info" />
                    <Input color={Color::Success} placeholder="Success" />
                    <Input color={Color::Warning} placeholder="Warning" />
                    <Input color={Color::Error} placeholder="Error" />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex flex-col gap-3 max-w-md">
                    <Input size={Size::ExtraSmall} placeholder="Extra Small" />
                    <Input size={Size::Small} placeholder="Small" />
                    <Input size={Size::Medium} placeholder="Medium" />
                    <Input size={Size::Large} placeholder="Large" />
                    <Input size={Size::ExtraLarge} placeholder="Extra Large" />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Styles and states"</h2>
                <div class="flex flex-col gap-3 max-w-md">
                    <Input variant={Variant::Outline} placeholder="Bordered / outline style" />
                    <Input variant={Variant::Ghost} placeholder="Ghost style" />
                    <Input disabled=true placeholder="Disabled input" />
                    <Input required=true placeholder="Required input" />
                    <Input readonly=true value="Read only value" />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"With labels and helper text"</h2>
                <div class="max-w-md space-y-4">
                    <Fieldset legend="Username">
                        <Input id="username-input" name="username" placeholder="Pick a username" />
                        <p class="label">"3-20 chars, letters and numbers only"</p>
                    </Fieldset>
                    <Fieldset legend="Email">
                        <Input id="email-input" name="email" input_type="email" placeholder="name@example.com" />
                    </Fieldset>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Reactive binding"</h2>
                <div class="max-w-md space-y-3">
                    <Input
                        placeholder="Type your username"
                        prop:value=move || username.get()
                        on:input=move |ev| set_username.set(event_target_value(&ev))
                    />
                    <Input
                        input_type="email"
                        placeholder="Type your email"
                        prop:value=move || email.get()
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />
                    <div class="text-sm text-base-content/70">
                        <p>{move || format!("username: {}", username.get())}</p>
                        <p>{move || format!("email: {}", email.get())}</p>
                    </div>
                </div>
            </section>
        </div>
    }
}
