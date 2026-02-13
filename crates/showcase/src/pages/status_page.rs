use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn StatusPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Status"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Visual status indicators with sizes, colors, and contextual usage patterns."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "status", type_label: "base", description: "Base class for status indicator dot" },
                    ClassEntry { name: "status-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "status-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "status-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "status-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "status-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "status-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "status-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "status-lg", type_label: "size", description: "Large size" },
                    ClassEntry { name: "status-md", type_label: "size", description: "Medium size (default)" },
                    ClassEntry { name: "status-sm", type_label: "size", description: "Small size" },
                    ClassEntry { name: "status-xs", type_label: "size", description: "Extra small size" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<Status size={Size::ExtraSmall} />
<Status size={Size::Small} />
<Status size={Size::Medium} />
<Status size={Size::Large} />
<Status size={Size::ExtraLarge} />"#
                >
                    <div class="flex flex-wrap gap-4 items-center">
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::ExtraSmall} />
                            <span class="text-xs text-base-content/70">"XS"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Small} />
                            <span class="text-xs text-base-content/70">"SM"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Medium} />
                            <span class="text-xs text-base-content/70">"Default"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Medium} />
                            <span class="text-xs text-base-content/70">"MD"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Large} />
                            <span class="text-xs text-base-content/70">"LG"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::ExtraLarge} />
                            <span class="text-xs text-base-content/70">"XL"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r#"<Status color={Color::Default} />
<Status color={Color::Primary} />
<Status color={Color::Secondary} />
<Status color={Color::Accent} />
<Status color={Color::Neutral} />
<Status color={Color::Info} />
<Status color={Color::Success} />
<Status color={Color::Warning} />
<Status color={Color::Error} />"#
                >
                    <div class="flex flex-wrap gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Default} />
                            <span class="text-xs text-base-content/70">"Default"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Primary} />
                            <span class="text-xs text-base-content/70">"Primary"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Secondary} />
                            <span class="text-xs text-base-content/70">"Secondary"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Accent} />
                            <span class="text-xs text-base-content/70">"Accent"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Neutral} />
                            <span class="text-xs text-base-content/70">"Neutral"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Info} />
                            <span class="text-xs text-base-content/70">"Info"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Success} />
                            <span class="text-xs text-base-content/70">"Success"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Warning} />
                            <span class="text-xs text-base-content/70">"Warning"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status color={Color::Error} />
                            <span class="text-xs text-base-content/70">"Error"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Combinations"
                    code=r#"<Status size={Size::Large} color={Color::Success} />
<Status size={Size::Large} color={Color::Error} />
<Status size={Size::Medium} color={Color::Warning} />
<Status size={Size::ExtraLarge} color={Color::Info} />"#
                >
                    <div class="flex flex-wrap gap-4">
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Large} color={Color::Success} />
                            <span class="text-xs text-base-content/70">"Large Success"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Large} color={Color::Error} />
                            <span class="text-xs text-base-content/70">"Large Error"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::Medium} color={Color::Warning} />
                            <span class="text-xs text-base-content/70">"Medium Warning"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Status size={Size::ExtraLarge} color={Color::Info} />
                            <span class="text-xs text-base-content/70">"XL Info"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Avatars"
                    code=r#"<div class="relative">
    <Avatar>
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Online" alt="User" />
    </Avatar>
    <div class="absolute bottom-0 right-0">
        <Status size={Size::Small} color={Color::Success} />
    </div>
</div>"#
                >
                    <div class="flex flex-wrap gap-4">
                        <div class="relative">
                            <Avatar>
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Online" alt="User" />
                            </Avatar>
                            <div class="absolute bottom-0 right-0">
                                <Status size={Size::Small} color={Color::Success} />
                            </div>
                        </div>
                        <div class="relative">
                            <Avatar>
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Away" alt="User" />
                            </Avatar>
                            <div class="absolute bottom-0 right-0">
                                <Status size={Size::Small} color={Color::Warning} />
                            </div>
                        </div>
                        <div class="relative">
                            <Avatar>
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Busy" alt="User" />
                            </Avatar>
                            <div class="absolute bottom-0 right-0">
                                <Status size={Size::Small} color={Color::Error} />
                            </div>
                        </div>
                        <div class="relative">
                            <Avatar>
                                <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Offline" alt="User" />
                            </Avatar>
                            <div class="absolute bottom-0 right-0">
                                <Status size={Size::Small} color={Color::Neutral} />
                            </div>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="In Context"
                    code=r#"<div class="flex items-center gap-3 p-2 bg-base-200 rounded">
    <Status size={Size::Small} color={Color::Success} />
    <span>"System operational"</span>
</div>
<div class="flex items-center gap-3 p-2 bg-base-200 rounded">
    <Status size={Size::Small} color={Color::Warning} />
    <span>"High CPU usage"</span>
</div>
<div class="flex items-center gap-3 p-2 bg-base-200 rounded">
    <Status size={Size::Small} color={Color::Error} />
    <span>"Service unavailable"</span>
</div>
<div class="flex items-center gap-3 p-2 bg-base-200 rounded">
    <Status size={Size::Small} color={Color::Info} />
    <span>"Update available"</span>
</div>"#
                >
                    <div class="space-y-2 max-w-md">
                        <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                            <Status size={Size::Small} color={Color::Success} />
                            <span>"System operational"</span>
                        </div>
                        <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                            <Status size={Size::Small} color={Color::Warning} />
                            <span>"High CPU usage"</span>
                        </div>
                        <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                            <Status size={Size::Small} color={Color::Error} />
                            <span>"Service unavailable"</span>
                        </div>
                        <div class="flex items-center gap-3 p-2 bg-base-200 rounded">
                            <Status size={Size::Small} color={Color::Info} />
                            <span>"Update available"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
