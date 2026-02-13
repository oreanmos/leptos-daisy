use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Button"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Complete daisyUI button variants with practical combinations and accessibility patterns."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "btn", type_label: "base", description: "Base button class" },
                    ClassEntry { name: "btn-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "btn-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "btn-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "btn-neutral", type_label: "color", description: "Neutral color" },
                    ClassEntry { name: "btn-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "btn-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "btn-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "btn-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "btn-ghost", type_label: "color", description: "Ghost color" },
                    ClassEntry { name: "btn-lg", type_label: "size", description: "Large size" },
                    ClassEntry { name: "btn-md", type_label: "size", description: "Medium size (default)" },
                    ClassEntry { name: "btn-sm", type_label: "size", description: "Small size" },
                    ClassEntry { name: "btn-xs", type_label: "size", description: "Extra small size" },
                    ClassEntry { name: "btn-outline", type_label: "style", description: "Outline style" },
                    ClassEntry { name: "btn-link", type_label: "style", description: "Link style" },
                    ClassEntry { name: "btn-ghost", type_label: "style", description: "Ghost style" },
                    ClassEntry { name: "btn-soft", type_label: "style", description: "Soft style" },
                    ClassEntry { name: "btn-dash", type_label: "style", description: "Dash style" },
                    ClassEntry { name: "btn-wide", type_label: "modifier", description: "Wide button" },
                    ClassEntry { name: "btn-block", type_label: "modifier", description: "Full width button" },
                    ClassEntry { name: "btn-circle", type_label: "modifier", description: "Circle shape" },
                    ClassEntry { name: "btn-square", type_label: "modifier", description: "Square shape" },
                    ClassEntry { name: "btn-active", type_label: "state", description: "Active state" },
                    ClassEntry { name: "btn-disabled", type_label: "state", description: "Disabled state" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r#"<Button>"Default"</Button>
<Button color={Color::Neutral}>"Neutral"</Button>
<Button color={Color::Primary}>"Primary"</Button>
<Button color={Color::Secondary}>"Secondary"</Button>
<Button color={Color::Accent}>"Accent"</Button>
<Button color={Color::Info}>"Info"</Button>
<Button color={Color::Success}>"Success"</Button>
<Button color={Color::Warning}>"Warning"</Button>
<Button color={Color::Error}>"Error"</Button>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Button>"Default"</Button>
                        <Button color={Color::Neutral}>"Neutral"</Button>
                        <Button color={Color::Primary}>"Primary"</Button>
                        <Button color={Color::Secondary}>"Secondary"</Button>
                        <Button color={Color::Accent}>"Accent"</Button>
                        <Button color={Color::Info}>"Info"</Button>
                        <Button color={Color::Success}>"Success"</Button>
                        <Button color={Color::Warning}>"Warning"</Button>
                        <Button color={Color::Error}>"Error"</Button>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Styles"
                    code=r#"<Button variant={Variant::Solid} color={Color::Primary}>"Solid"</Button>
<Button variant={Variant::Outline} color={Color::Primary}>"Outline"</Button>
<Button variant={Variant::Ghost}>"Ghost"</Button>
<Button variant={Variant::Link}>"Link"</Button>
<Button variant={Variant::Soft} color={Color::Primary}>"Soft"</Button>
<Button variant={Variant::Dash}>"Dash"</Button>
<Button glass=true color={Color::Secondary}>"Glass"</Button>
<Button no_animation=true color={Color::Accent}>"No animation"</Button>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Button variant={Variant::Solid} color={Color::Primary}>"Solid"</Button>
                        <Button variant={Variant::Outline} color={Color::Primary}>"Outline"</Button>
                        <Button variant={Variant::Ghost}>"Ghost"</Button>
                        <Button variant={Variant::Link}>"Link"</Button>
                        <Button variant={Variant::Soft} color={Color::Primary}>"Soft"</Button>
                        <Button variant={Variant::Dash}>"Dash"</Button>
                        <Button glass=true color={Color::Secondary}>"Glass"</Button>
                        <Button no_animation=true color={Color::Accent}>"No animation"</Button>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes and shape"
                    code=r#"<Button size={Size::ExtraSmall}>"XS"</Button>
<Button size={Size::Small}>"Small"</Button>
<Button size={Size::Medium}>"Medium"</Button>
<Button size={Size::Large}>"Large"</Button>
<Button size={Size::ExtraLarge}>"XL"</Button>
<Button square=true color={Color::Primary}>"■"</Button>
<Button circle=true color={Color::Secondary}>"●"</Button>"#
                >
                    <div class="flex flex-wrap items-center gap-2">
                        <Button size={Size::ExtraSmall}>"XS"</Button>
                        <Button size={Size::Small}>"Small"</Button>
                        <Button size={Size::Medium}>"Medium"</Button>
                        <Button size={Size::Large}>"Large"</Button>
                        <Button size={Size::ExtraLarge}>"XL"</Button>
                        <Button square=true color={Color::Primary}>
                            "■"
                        </Button>
                        <Button circle=true color={Color::Secondary}>
                            "●"
                        </Button>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="State"
                    code=r#"<Button state={State::Active}>"Active"</Button>
<Button state={State::Loading}>"Loading"</Button>
<Button disabled=true>"Disabled"</Button>
<Button color={Color::Primary}>
    <span class="loading loading-spinner loading-xs mr-1"></span>
    "With spinner"
</Button>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Button state={State::Active}>"Active"</Button>
                        <Button state={State::Loading}>"Loading"</Button>
                        <Button disabled=true>"Disabled"</Button>
                        <Button color={Color::Primary}>
                            <span class="loading loading-spinner loading-xs mr-1"></span>
                            "With spinner"
                        </Button>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Layout patterns"
                    code=r#"<Button wide=true color={Color::Primary}>"Wide Button"</Button>
<Button block=true color={Color::Secondary}>"Block Button"</Button>

<div class="join">
    <Button class="join-item" variant={Variant::Outline}>"Previous"</Button>
    <Button class="join-item" color={Color::Primary}>"Current"</Button>
    <Button class="join-item" variant={Variant::Outline}>"Next"</Button>
</div>"#
                >
                    <div class="space-y-3 max-w-xl">
                        <Button wide=true color={Color::Primary}>"Wide Button"</Button>
                        <Button block=true color={Color::Secondary}>"Block Button"</Button>
                        <div class="join">
                            <Button class="join-item" variant={Variant::Outline}>"Previous"</Button>
                            <Button class="join-item" color={Color::Primary}>"Current"</Button>
                            <Button class="join-item" variant={Variant::Outline}>"Next"</Button>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
