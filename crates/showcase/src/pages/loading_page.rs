use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Loading"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Loading indicators with multiple variants, sizes, and colors for showing async states."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "loading", type_label: "base", description: "Base class for loading indicator" },
                    ClassEntry { name: "loading-spinner", type_label: "modifier", description: "Spinner variant animation" },
                    ClassEntry { name: "loading-dots", type_label: "modifier", description: "Dots variant animation" },
                    ClassEntry { name: "loading-ring", type_label: "modifier", description: "Ring variant animation" },
                    ClassEntry { name: "loading-ball", type_label: "modifier", description: "Ball variant animation" },
                    ClassEntry { name: "loading-bars", type_label: "modifier", description: "Bars variant animation" },
                    ClassEntry { name: "loading-infinity", type_label: "modifier", description: "Infinity variant animation" },
                    ClassEntry { name: "loading-lg", type_label: "size", description: "Large size loading indicator" },
                    ClassEntry { name: "loading-md", type_label: "size", description: "Medium size loading indicator" },
                    ClassEntry { name: "loading-sm", type_label: "size", description: "Small size loading indicator" },
                    ClassEntry { name: "loading-xs", type_label: "size", description: "Extra small size loading indicator" },
                    ClassEntry { name: "text-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "text-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "text-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "text-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "text-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "text-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "text-error", type_label: "color", description: "Error color" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Variants"
                    code=r#"<Loading variant={LoadingVariant::Spinner} />
<Loading variant={LoadingVariant::Dots} />
<Loading variant={LoadingVariant::Ring} />
<Loading variant={LoadingVariant::Ball} />
<Loading variant={LoadingVariant::Bars} />
<Loading variant={LoadingVariant::Infinity} />"#
                >
                    <div class="flex flex-wrap gap-8 items-center">
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Spinner} />
                            <span class="text-sm text-base-content/70">"Spinner"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Dots} />
                            <span class="text-sm text-base-content/70">"Dots"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Ring} />
                            <span class="text-sm text-base-content/70">"Ring"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Ball} />
                            <span class="text-sm text-base-content/70">"Ball"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Bars} />
                            <span class="text-sm text-base-content/70">"Bars"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Infinity} />
                            <span class="text-sm text-base-content/70">"Infinity"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<Loading size={Size::ExtraSmall} />
<Loading size={Size::Small} />
<Loading size={Size::Medium} />
<Loading size={Size::Large} />
<Loading size={Size::ExtraLarge} />"#
                >
                    <div class="space-y-6">
                        <div class="flex items-center gap-4">
                            <Loading size={Size::ExtraSmall} />
                            <span class="text-sm text-base-content/70">"Extra Small"</span>
                        </div>
                        <div class="flex items-center gap-4">
                            <Loading size={Size::Small} />
                            <span class="text-sm text-base-content/70">"Small"</span>
                        </div>
                        <div class="flex items-center gap-4">
                            <Loading size={Size::Medium} />
                            <span class="text-sm text-base-content/70">"Medium"</span>
                        </div>
                        <div class="flex items-center gap-4">
                            <Loading size={Size::Large} />
                            <span class="text-sm text-base-content/70">"Large"</span>
                        </div>
                        <div class="flex items-center gap-4">
                            <Loading size={Size::ExtraLarge} />
                            <span class="text-sm text-base-content/70">"Extra Large"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r#"<Loading color={Color::Primary} />
<Loading color={Color::Secondary} />
<Loading color={Color::Accent} />
<Loading color={Color::Info} />
<Loading color={Color::Success} />
<Loading color={Color::Warning} />
<Loading color={Color::Error} />"#
                >
                    <div class="flex flex-wrap gap-8 items-center">
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Primary} />
                            <span class="text-sm text-base-content/70">"Primary"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Secondary} />
                            <span class="text-sm text-base-content/70">"Secondary"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Accent} />
                            <span class="text-sm text-base-content/70">"Accent"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Info} />
                            <span class="text-sm text-base-content/70">"Info"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Success} />
                            <span class="text-sm text-base-content/70">"Success"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Warning} />
                            <span class="text-sm text-base-content/70">"Warning"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading color={Color::Error} />
                            <span class="text-sm text-base-content/70">"Error"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Combinations"
                    code=r#"<Loading variant={LoadingVariant::Spinner} size={Size::Large} color={Color::Primary} />
<Loading variant={LoadingVariant::Dots} size={Size::ExtraLarge} color={Color::Secondary} />
<Loading variant={LoadingVariant::Ring} size={Size::Medium} color={Color::Accent} />"#
                >
                    <div class="flex flex-wrap gap-8 items-center">
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Spinner} size={Size::Large} color={Color::Primary} />
                            <span class="text-sm text-base-content/70">"Spinner Large Primary"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Dots} size={Size::ExtraLarge} color={Color::Secondary} />
                            <span class="text-sm text-base-content/70">"Dots XL Secondary"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Loading variant={LoadingVariant::Ring} size={Size::Medium} color={Color::Accent} />
                            <span class="text-sm text-base-content/70">"Ring Medium Accent"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
