use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DividerPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Divider"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Divider components for separating content horizontally or vertically with optional text labels."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "divider", type_label: "base", description: "Base divider class" },
                    ClassEntry { name: "divider-horizontal", type_label: "modifier", description: "Horizontal divider" },
                    ClassEntry { name: "divider-vertical", type_label: "modifier", description: "Vertical divider" },
                    ClassEntry { name: "divider-primary", type_label: "color", description: "Primary color" },
                    ClassEntry { name: "divider-secondary", type_label: "color", description: "Secondary color" },
                    ClassEntry { name: "divider-accent", type_label: "color", description: "Accent color" },
                    ClassEntry { name: "divider-neutral", type_label: "color", description: "Neutral color" },
                    ClassEntry { name: "divider-info", type_label: "color", description: "Info color" },
                    ClassEntry { name: "divider-success", type_label: "color", description: "Success color" },
                    ClassEntry { name: "divider-warning", type_label: "color", description: "Warning color" },
                    ClassEntry { name: "divider-error", type_label: "color", description: "Error color" },
                    ClassEntry { name: "divider-start", type_label: "modifier", description: "Text at start" },
                    ClassEntry { name: "divider-end", type_label: "modifier", description: "Text at end" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Horizontal Divider"
                    code=r#"<p>"Content above the divider"</p>
<Divider />
<p>"Content below the divider"</p>"#
                >
                    <div class="space-y-4">
                        <p>"Content above the divider"</p>
                        <Divider />
                        <p>"Content below the divider"</p>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Vertical Divider"
                    code=r#"<div class="flex h-24">
    <div class="flex-1 flex items-center justify-center">"Left content"</div>
    <Divider orientation={DividerOrientation::Vertical} />
    <div class="flex-1 flex items-center justify-center">"Right content"</div>
</div>"#
                >
                    <div class="flex h-24">
                        <div class="flex-1 flex items-center justify-center">"Left content"</div>
                        <Divider orientation={DividerOrientation::Vertical} />
                        <div class="flex-1 flex items-center justify-center">"Right content"</div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Divider with Text"
                    code=r#"<p>"Some content here"</p>
<Divider text="OR".to_string() />
<p>"More content here"</p>"#
                >
                    <div class="space-y-4">
                        <p>"Some content here"</p>
                        <Divider text="OR".to_string() />
                        <p>"More content here"</p>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Divider Colors"
                    code=r#"<Divider color={Color::Primary} text="Primary".to_string() />
<Divider color={Color::Secondary} text="Secondary".to_string() />
<Divider color={Color::Accent} text="Accent".to_string() />
<Divider color={Color::Neutral} text="Neutral".to_string() />
<Divider color={Color::Info} text="Info".to_string() />
<Divider color={Color::Success} text="Success".to_string() />
<Divider color={Color::Warning} text="Warning".to_string() />
<Divider color={Color::Error} text="Error".to_string() />"#
                >
                    <div class="space-y-4">
                        <Divider color={Color::Primary} text="Primary".to_string() />
                        <Divider color={Color::Secondary} text="Secondary".to_string() />
                        <Divider color={Color::Accent} text="Accent".to_string() />
                        <Divider color={Color::Neutral} text="Neutral".to_string() />
                        <Divider color={Color::Info} text="Info".to_string() />
                        <Divider color={Color::Success} text="Success".to_string() />
                        <Divider color={Color::Warning} text="Warning".to_string() />
                        <Divider color={Color::Error} text="Error".to_string() />
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Vertical with Colors"
                    code=r#"<div class="flex h-24 gap-4">
    <div class="flex-1 flex items-center justify-center">"Content"</div>
    <Divider orientation={DividerOrientation::Vertical} color={Color::Primary} />
    <div class="flex-1 flex items-center justify-center">"Content"</div>
    <Divider orientation={DividerOrientation::Vertical} color={Color::Secondary} />
    <div class="flex-1 flex items-center justify-center">"Content"</div>
    <Divider orientation={DividerOrientation::Vertical} color={Color::Accent} />
    <div class="flex-1 flex items-center justify-center">"Content"</div>
</div>"#
                >
                    <div class="flex h-24 gap-4">
                        <div class="flex-1 flex items-center justify-center">"Content"</div>
                        <Divider orientation={DividerOrientation::Vertical} color={Color::Primary} />
                        <div class="flex-1 flex items-center justify-center">"Content"</div>
                        <Divider orientation={DividerOrientation::Vertical} color={Color::Secondary} />
                        <div class="flex-1 flex items-center justify-center">"Content"</div>
                        <Divider orientation={DividerOrientation::Vertical} color={Color::Accent} />
                        <div class="flex-1 flex items-center justify-center">"Content"</div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="In Context"
                    code=r#"<Card class="bg-base-100 shadow-xl max-w-md">
    <CardBody>
        <CardTitle>"Sign In"</CardTitle>
        <div class="space-y-4">
            <Input placeholder="Email" />
            <Input placeholder="Password" />
            <Button color={Color::Primary} class="w-full">"Sign In"</Button>
        </div>
        <Divider text="OR".to_string() />
        <div class="flex gap-2">
            <Button variant={Variant::Outline} class="flex-1">"Google"</Button>
            <Button variant={Variant::Outline} class="flex-1">"GitHub"</Button>
        </div>
    </CardBody>
</Card>"#
                >
                    <Card class="bg-base-100 shadow-xl max-w-md">
                        <CardBody>
                            <CardTitle>"Sign In"</CardTitle>
                            <div class="space-y-4">
                                <Input placeholder="Email" />
                                <Input placeholder="Password" />
                                <Button color={Color::Primary} class="w-full">"Sign In"</Button>
                            </div>
                            <Divider text="OR".to_string() />
                            <div class="flex gap-2">
                                <Button variant={Variant::Outline} class="flex-1">"Google"</Button>
                                <Button variant={Variant::Outline} class="flex-1">"GitHub"</Button>
                            </div>
                        </CardBody>
                    </Card>
                </ComponentPreview>
            </section>
        </div>
    }
}
