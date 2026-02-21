use crate::components::{ClassEntry, ClassTable, ComponentPreview};
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
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "file-input", type_label: "base", description: "Base file input component" },
                    ClassEntry { name: "file-input-bordered", type_label: "modifier", description: "Adds border to file input" },
                    ClassEntry { name: "file-input-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "file-input-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "file-input-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "file-input-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "file-input-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "file-input-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "file-input-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "file-input-ghost", type_label: "color", description: "Ghost style" },
                    ClassEntry { name: "file-input-lg", type_label: "size", description: "Large size" },
                    ClassEntry { name: "file-input-md", type_label: "size", description: "Medium size" },
                    ClassEntry { name: "file-input-sm", type_label: "size", description: "Small size" },
                    ClassEntry { name: "file-input-xs", type_label: "size", description: "Extra small size" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<FileInput size={Size::ExtraSmall} />
<FileInput size={Size::Small} />
<FileInput size={Size::Medium} />
<FileInput size={Size::Large} />
<FileInput size={Size::ExtraLarge} />"##
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<FileInput color={Color::Primary} />
<FileInput color={Color::Secondary} />
<FileInput color={Color::Accent} />
<FileInput color={Color::Info} />
<FileInput color={Color::Success} />
<FileInput color={Color::Warning} />
<FileInput color={Color::Error} />"##
                >
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
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Styles and states"
                    code=r##"<FileInput variant={FileInputVariant::Bordered} />
<FileInput variant={FileInputVariant::Ghost} />
<FileInput variant={FileInputVariant::Bordered} color={Color::Primary} />
<FileInput disabled=true />"##
                >
                    <div class="space-y-3 max-w-xl">
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Bordered"</span>
                            <FileInput variant={FileInputVariant::Bordered} />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Ghost"</span>
                            <FileInput variant={FileInputVariant::Ghost} />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Bordered + Primary Color"</span>
                            <FileInput variant={FileInputVariant::Bordered} color={Color::Primary} />
                        </div>
                        <div class="space-y-1">
                            <span class="text-sm text-base-content/70">"Disabled"</span>
                            <FileInput disabled=true />
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Form attributes"
                    code=r##"<FileInput
    id="avatar-upload"
    name="avatar"
    accept="image/png,image/jpeg"
    aria_label="Upload profile image"
/>

// Reactive example
let (picked_path, set_picked_path) = signal(String::new());

view! {
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
}"##
                >
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
                </ComponentPreview>
            </section>
        </div>
    }
}
