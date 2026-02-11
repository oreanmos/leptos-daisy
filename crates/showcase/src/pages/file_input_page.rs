use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FileInputPage() -> impl IntoView {
    let (picked_path, set_picked_path) = signal(String::new());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"File Input"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "File upload controls with full daisyUI sizes, colors, and style modifiers."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="space-y-3 max-w-xl">
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <FileInput size={Size::ExtraSmall} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <FileInput size={Size::Small} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Medium"</span>
                        <FileInput size={Size::Medium} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <FileInput size={Size::Large} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Extra Large"</span>
                        <FileInput size={Size::ExtraLarge} />
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="space-y-3 max-w-xl">
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Primary"</span>
                        <FileInput color={Color::Primary} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Secondary"</span>
                        <FileInput color={Color::Secondary} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Accent"</span>
                        <FileInput color={Color::Accent} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Info"</span>
                        <FileInput color={Color::Info} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Success"</span>
                        <FileInput color={Color::Success} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Warning"</span>
                        <FileInput color={Color::Warning} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Error"</span>
                        <FileInput color={Color::Error} />
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Styles and states"</h2>
                <div class="space-y-3 max-w-xl">
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Bordered"</span>
                        <FileInput bordered=true />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Ghost"</span>
                        <FileInput ghost=true />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Bordered + Primary Color"</span>
                        <FileInput bordered=true color={Color::Primary} />
                    </div>
                    <div class="space-y-1">
                        <span class="text-sm text-base-content/70">"Disabled"</span>
                        <FileInput disabled=true />
                    </div>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Form attributes"</h2>
                <div class="space-y-3 max-w-xl">
                    <FileInput
                        id="avatar-upload"
                        name="avatar"
                        accept="image/png,image/jpeg"
                        aria_label="Upload profile image"
                    />
                    <FileInput
                        name="attachments"
                        multiple=true
                        required=true
                        on:change=move |ev| set_picked_path.set(event_target_value(&ev))
                    />
                    <p class="text-sm text-base-content/70">
                        {move || {
                            if picked_path.get().is_empty() {
                                "No file selected yet".to_string()
                            } else {
                                format!("Latest selection: {}", picked_path.get())
                            }
                        }}
                    </p>
                </div>
            </section>
        </div>
    }
}
