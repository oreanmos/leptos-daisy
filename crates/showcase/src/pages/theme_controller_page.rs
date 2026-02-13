use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ThemeControllerPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Theme Controller"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "CSS-only theme switcher inputs. Combine with toggle, checkbox, radio, or button classes to match your UI."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable
                    entries=vec![
                        ClassEntry { name: "theme-controller", type_label: "base", description: "Theme switch input class" },
                        ClassEntry { name: "toggle theme-controller", type_label: "style", description: "Toggle-style controller" },
                        ClassEntry { name: "checkbox theme-controller", type_label: "style", description: "Checkbox-style controller" },
                        ClassEntry { name: "radio theme-controller", type_label: "style", description: "Radio-style controller" },
                        ClassEntry { name: "btn theme-controller", type_label: "style", description: "Button-style controller" },
                    ]
                />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="As toggle and checkbox"
                    code=r#"<div class=\"flex items-center gap-4\">
    <ThemeController value=\"dark\" class=\"toggle\" aria_label=\"Enable dark theme\" />
    <ThemeController value=\"cupcake\" class=\"checkbox\" aria_label=\"Use cupcake theme\" />
</div>"#
                >
                    <div class="flex flex-wrap items-center gap-4">
                        <ThemeController value="dark" class="toggle" aria_label="Enable dark theme" />
                        <ThemeController value="cupcake" class="checkbox" aria_label="Use cupcake theme" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Radio group theme picker"
                    code=r#"<div class=\"flex items-center gap-4\">
    <ThemeController value=\"light\" input_type=ThemeControllerType::Radio name=\"theme-picker\" class=\"radio\" aria_label=\"Light\" checked=true />
    <ThemeController value=\"dark\" input_type=ThemeControllerType::Radio name=\"theme-picker\" class=\"radio\" aria_label=\"Dark\" />
    <ThemeController value=\"synthwave\" input_type=ThemeControllerType::Radio name=\"theme-picker\" class=\"radio\" aria_label=\"Synthwave\" />
</div>"#
                >
                    <div class="flex flex-wrap items-center gap-4">
                        <ThemeController
                            value="light"
                            input_type=ThemeControllerType::Radio
                            name="theme-picker"
                            class="radio"
                            aria_label="Light"
                            checked=true
                        />
                        <ThemeController
                            value="dark"
                            input_type=ThemeControllerType::Radio
                            name="theme-picker"
                            class="radio"
                            aria_label="Dark"
                        />
                        <ThemeController
                            value="synthwave"
                            input_type=ThemeControllerType::Radio
                            name="theme-picker"
                            class="radio"
                            aria_label="Synthwave"
                        />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Button style + default reset"
                    code=r#"<div class=\"join\">
    <ThemeController value=\"retro\" input_type=ThemeControllerType::Radio name=\"theme-btns\" class=\"btn join-item\" aria_label=\"Retro\" />
    <ThemeController value=\"night\" input_type=ThemeControllerType::Radio name=\"theme-btns\" class=\"btn join-item\" aria_label=\"Night\" />
    <ThemeController value=\"default\" input_type=ThemeControllerType::Radio name=\"theme-btns\" class=\"btn join-item\" aria_label=\"Reset to default\" />
</div>"#
                >
                    <div class="join">
                        <ThemeController
                            value="retro"
                            input_type=ThemeControllerType::Radio
                            name="theme-btns"
                            class="btn join-item"
                            aria_label="Retro"
                        />
                        <ThemeController
                            value="night"
                            input_type=ThemeControllerType::Radio
                            name="theme-btns"
                            class="btn join-item"
                            aria_label="Night"
                        />
                        <ThemeController
                            value="default"
                            input_type=ThemeControllerType::Radio
                            name="theme-btns"
                            class="btn join-item"
                            aria_label="Reset to default"
                        />
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
