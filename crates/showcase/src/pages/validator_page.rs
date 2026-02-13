use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ValidatorPage() -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let username_too_short = Signal::derive(move || username.get().chars().count() < 3);
    let username_class = Signal::derive(move || {
        if username_too_short.get() {
            "input-error".to_string()
        } else {
            "input-success".to_string()
        }
    });

    let (email, set_email) = signal(String::new());
    let invalid_email = Signal::derive(move || {
        let current = email.get();
        !current.is_empty() && !current.contains('@')
    });
    let email_class = Signal::derive(move || {
        if invalid_email.get() {
            "input-error".to_string()
        } else {
            String::new()
        }
    });

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Validator"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Validation message wrapper classes composed with native HTML constraints and reactive hints."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Required field pattern"
                    code=r##"<Fieldset legend="Display name">
    <Validator>
        <Input required=true placeholder="Required field" />
        <ValidatorLabel>"This field is required"</ValidatorLabel>
    </Validator>
</Fieldset>"##
                >
                    <div class="max-w-md">
                        <Fieldset legend="Display name">
                            <Validator>
                                <Input required=true placeholder="Required field" />
                                <ValidatorLabel>"This field is required"</ValidatorLabel>
                            </Validator>
                        </Fieldset>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive username rule"
                    code=r##"let (username, set_username) = signal(String::new());
let username_too_short = Signal::derive(move || username.get().chars().count() < 3);
let username_class = Signal::derive(move || {
    if username_too_short.get() { "input-error".to_string() } else { "input-success".to_string() }
});

view! {
    <Fieldset legend="Username (min 3 chars)">
        <Validator>
            <Input
                value=username
                class=username_class
                on:input=move |ev| set_username.set(event_target_value(&ev))
            />
            <ValidatorLabel>
                {move || if username_too_short.get() { "Must be at least 3 characters" } else { "Looks good" }}
            </ValidatorLabel>
        </Validator>
    </Fieldset>
}"##
                >
                    <div class="max-w-md">
                        <Fieldset legend="Username (min 3 chars)">
                            <Validator>
                                <Input
                                    value=username
                                    class=username_class
                                    on:input=move |ev| set_username.set(event_target_value(&ev))
                                />
                                <ValidatorLabel>
                                    {move || {
                                        if username_too_short.get() {
                                            "Must be at least 3 characters".to_string()
                                        } else {
                                            "Looks good".to_string()
                                        }
                                    }}
                                </ValidatorLabel>
                            </Validator>
                        </Fieldset>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive email hint"
                    code=r##"let (email, set_email) = signal(String::new());
let invalid_email = Signal::derive(move || {
    let current = email.get();
    !current.is_empty() && !current.contains('@')
});
let email_class = Signal::derive(move || {
    if invalid_email.get() { "input-error".to_string() } else { String::new() }
});

view! {
    <Fieldset legend="Email">
        <Validator>
            <Input
                input_type="email"
                value=email
                class=email_class
                on:input=move |ev| set_email.set(event_target_value(&ev))
            />
            <ValidatorLabel>
                {move || if invalid_email.get() {
                    "Please enter a valid email address"
                } else {
                    "We'll only use this for account notices"
                }}
            </ValidatorLabel>
        </Validator>
    </Fieldset>
}"##
                >
                    <div class="max-w-md">
                        <Fieldset legend="Email">
                            <Validator>
                                <Input
                                    input_type="email"
                                    value=email
                                    class=email_class
                                    on:input=move |ev| set_email.set(event_target_value(&ev))
                                />
                                <ValidatorLabel>
                                    {move || {
                                        if invalid_email.get() {
                                            "Please enter a valid email address".to_string()
                                        } else {
                                            "We'll only use this for account notices".to_string()
                                        }
                                    }}
                                </ValidatorLabel>
                            </Validator>
                        </Fieldset>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
