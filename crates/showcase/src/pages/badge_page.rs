use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn BadgePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Badge"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Badges are used to inform the user of the status of specific data."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "badge", type_label: "base", description: "Container for badge content" },
                    ClassEntry { name: "badge-primary", type_label: "color", description: "Badge with primary color" },
                    ClassEntry { name: "badge-secondary", type_label: "color", description: "Badge with secondary color" },
                    ClassEntry { name: "badge-accent", type_label: "color", description: "Badge with accent color" },
                    ClassEntry { name: "badge-neutral", type_label: "color", description: "Badge with neutral color" },
                    ClassEntry { name: "badge-info", type_label: "color", description: "Badge with info color" },
                    ClassEntry { name: "badge-success", type_label: "color", description: "Badge with success color" },
                    ClassEntry { name: "badge-warning", type_label: "color", description: "Badge with warning color" },
                    ClassEntry { name: "badge-error", type_label: "color", description: "Badge with error color" },
                    ClassEntry { name: "badge-ghost", type_label: "color", description: "Badge with ghost color" },
                    ClassEntry { name: "badge-lg", type_label: "size", description: "Large badge size" },
                    ClassEntry { name: "badge-md", type_label: "size", description: "Medium badge size (default)" },
                    ClassEntry { name: "badge-sm", type_label: "size", description: "Small badge size" },
                    ClassEntry { name: "badge-xs", type_label: "size", description: "Extra small badge size" },
                    ClassEntry { name: "badge-outline", type_label: "style", description: "Badge with outline style" },
                    ClassEntry { name: "badge-soft", type_label: "style", description: "Badge with soft background style" },
                    ClassEntry { name: "badge-dash", type_label: "style", description: "Badge with dashed border style" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r#"<Badge>"Default"</Badge>
<Badge color={Color::Primary}>"Primary"</Badge>
<Badge color={Color::Secondary}>"Secondary"</Badge>
<Badge color={Color::Accent}>"Accent"</Badge>
<Badge color={Color::Info}>"Info"</Badge>
<Badge color={Color::Success}>"Success"</Badge>
<Badge color={Color::Warning}>"Warning"</Badge>
<Badge color={Color::Error}>"Error"</Badge>
<Badge color={Color::Neutral}>"Neutral"</Badge>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Badge>"Default"</Badge>
                        <Badge color={Color::Primary}>"Primary"</Badge>
                        <Badge color={Color::Secondary}>"Secondary"</Badge>
                        <Badge color={Color::Accent}>"Accent"</Badge>
                        <Badge color={Color::Info}>"Info"</Badge>
                        <Badge color={Color::Success}>"Success"</Badge>
                        <Badge color={Color::Warning}>"Warning"</Badge>
                        <Badge color={Color::Error}>"Error"</Badge>
                        <Badge color={Color::Neutral}>"Neutral"</Badge>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<Badge size={Size::ExtraSmall}>"XS"</Badge>
<Badge size={Size::Small}>"Small"</Badge>
<Badge size={Size::Medium}>"Medium"</Badge>
<Badge size={Size::Large}>"Large"</Badge>
<Badge size={Size::ExtraLarge}>"XL"</Badge>"#
                >
                    <div class="flex flex-wrap items-center gap-2">
                        <Badge size={Size::ExtraSmall}>"XS"</Badge>
                        <Badge size={Size::Small}>"Small"</Badge>
                        <Badge size={Size::Medium}>"Medium"</Badge>
                        <Badge size={Size::Large}>"Large"</Badge>
                        <Badge size={Size::ExtraLarge}>"XL"</Badge>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Variants"
                    code=r#"<Badge variant={Variant::Outline} color={Color::Primary}>"Outline"</Badge>
<Badge variant={Variant::Soft} color={Color::Secondary}>"Soft"</Badge>
<Badge variant={Variant::Dash}>"Dash"</Badge>
<Badge variant={Variant::Ghost}>"Ghost"</Badge>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Badge variant={Variant::Outline} color={Color::Primary}>"Outline"</Badge>
                        <Badge variant={Variant::Soft} color={Color::Secondary}>"Soft"</Badge>
                        <Badge variant={Variant::Dash}>"Dash"</Badge>
                        <Badge variant={Variant::Ghost}>"Ghost"</Badge>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Outline"
                    code=r#"<Badge variant={Variant::Outline}>"Default"</Badge>
<Badge variant={Variant::Outline} color={Color::Primary}>"Primary"</Badge>
<Badge variant={Variant::Outline} color={Color::Secondary}>"Secondary"</Badge>
<Badge variant={Variant::Outline} color={Color::Accent}>"Accent"</Badge>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Badge variant={Variant::Outline}>"Default"</Badge>
                        <Badge variant={Variant::Outline} color={Color::Primary}>"Primary"</Badge>
                        <Badge variant={Variant::Outline} color={Color::Secondary}>"Secondary"</Badge>
                        <Badge variant={Variant::Outline} color={Color::Accent}>"Accent"</Badge>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Empty Badge"
                    code=r#"<Badge size={Size::ExtraSmall} class="bg-primary" />
<Badge size={Size::Small} class="bg-secondary" />
<Badge size={Size::Medium} class="bg-accent" />
<Badge size={Size::Large} class="bg-info" />"#
                >
                    <div class="flex flex-wrap gap-2">
                        <Badge size={Size::ExtraSmall} class="bg-primary" />
                        <Badge size={Size::Small} class="bg-secondary" />
                        <Badge size={Size::Medium} class="bg-accent" />
                        <Badge size={Size::Large} class="bg-info" />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="In a button"
                    code=r#"<button class="btn">
    "Inbox"
    <Badge>"+99"</Badge>
</button>
<button class="btn">
    "Notifications"
    <Badge color={Color::Secondary}>"+12"</Badge>
</button>"#
                >
                    <div class="flex flex-wrap gap-2">
                        <button class="btn">
                            "Inbox"
                            <Badge>"+99"</Badge>
                        </button>
                        <button class="btn">
                            "Notifications"
                            <Badge color={Color::Secondary}>"+12"</Badge>
                        </button>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
