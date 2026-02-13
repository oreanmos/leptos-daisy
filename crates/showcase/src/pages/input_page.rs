use crate::components::{ClassEntry, ClassTable, ComponentPreview};
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
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "input", type_label: "base", description: "Base input component" },
                    ClassEntry { name: "input-bordered", type_label: "modifier", description: "Adds border to input" },
                    ClassEntry { name: "input-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "input-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "input-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "input-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "input-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "input-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "input-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "input-ghost", type_label: "color", description: "Ghost style" },
                    ClassEntry { name: "input-lg", type_label: "size", description: "Large size" },
                    ClassEntry { name: "input-md", type_label: "size", description: "Medium size" },
                    ClassEntry { name: "input-sm", type_label: "size", description: "Small size" },
                    ClassEntry { name: "input-xs", type_label: "size", description: "Extra small size" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<Input placeholder="Default input" />
<Input color={Color::Neutral} placeholder="Neutral" />
<Input color={Color::Primary} placeholder="Primary" />
<Input color={Color::Secondary} placeholder="Secondary" />
<Input color={Color::Accent} placeholder="Accent" />
<Input color={Color::Info} placeholder="Info" />
<Input color={Color::Success} placeholder="Success" />
<Input color={Color::Warning} placeholder="Warning" />
<Input color={Color::Error} placeholder="Error" />"##
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Input size={Size::ExtraSmall} placeholder="Extra Small" />
<Input size={Size::Small} placeholder="Small" />
<Input size={Size::Medium} placeholder="Medium" />
<Input size={Size::Large} placeholder="Large" />
<Input size={Size::ExtraLarge} placeholder="Extra Large" />"##
                >
                    <div class="flex flex-col gap-3 max-w-md">
                        <Input size={Size::ExtraSmall} placeholder="Extra Small" />
                        <Input size={Size::Small} placeholder="Small" />
                        <Input size={Size::Medium} placeholder="Medium" />
                        <Input size={Size::Large} placeholder="Large" />
                        <Input size={Size::ExtraLarge} placeholder="Extra Large" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Styles and states"
                    code=r##"<Input variant={Variant::Outline} placeholder="Bordered / outline style" />
<Input variant={Variant::Ghost} placeholder="Ghost style" />
<Input disabled=true placeholder="Disabled input" />
<Input required=true placeholder="Required input" />
<Input readonly=true value="Read only value" />"##
                >
                    <div class="flex flex-col gap-3 max-w-md">
                        <Input variant={Variant::Outline} placeholder="Bordered / outline style" />
                        <Input variant={Variant::Ghost} placeholder="Ghost style" />
                        <Input disabled=true placeholder="Disabled input" />
                        <Input required=true placeholder="Required input" />
                        <Input readonly=true value="Read only value" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With labels and helper text"
                    code=r##"<Fieldset legend="Username">
    <Input id="username-input" name="username" placeholder="Pick a username" />
    <p class="label">"3-20 chars, letters and numbers only"</p>
</Fieldset>

<Fieldset legend="Email">
    <Input id="email-input" name="email" input_type="email" placeholder="name@example.com" />
</Fieldset>"##
                >
                    <div class="max-w-md space-y-4">
                        <Fieldset legend="Username">
                            <Input id="username-input" name="username" placeholder="Pick a username" />
                            <p class="label">"3-20 chars, letters and numbers only"</p>
                        </Fieldset>
                        <Fieldset legend="Email">
                            <Input id="email-input" name="email" input_type="email" placeholder="name@example.com" />
                        </Fieldset>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive binding"
                    code=r##"let (username, set_username) = signal(String::new());
let (email, set_email) = signal(String::new());

view! {
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
}"##
                >
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
                </ComponentPreview>
            </section>
        </div>
    }
}
