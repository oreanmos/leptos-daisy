use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TabPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Tabs"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "DaisyUI tabs with style variants, sizes, and panel composition examples."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "tabs", type_label: "base", description: "Base tabs container" },
                    ClassEntry { name: "tab", type_label: "base part", description: "Individual tab" },
                    ClassEntry { name: "tab-content", type_label: "base part", description: "Content panel for tab" },
                    ClassEntry { name: "tabs-border", type_label: "style", description: "Bordered tabs style" },
                    ClassEntry { name: "tabs-lift", type_label: "style", description: "Lifted tabs style" },
                    ClassEntry { name: "tabs-box", type_label: "style", description: "Boxed tabs style" },
                    ClassEntry { name: "tabs-lg", type_label: "size", description: "Large" },
                    ClassEntry { name: "tabs-md", type_label: "size", description: "Medium (default)" },
                    ClassEntry { name: "tabs-sm", type_label: "size", description: "Small" },
                    ClassEntry { name: "tabs-xs", type_label: "size", description: "Extra small" },
                    ClassEntry { name: "tab-active", type_label: "state", description: "Active tab" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Default"
                    code=r#"<Tabs>
    <Tab active=true>"Overview"</Tab>
    <Tab>"Analytics"</Tab>
    <Tab>"Reports"</Tab>
    <Tab disabled=true>"Disabled"</Tab>
</Tabs>"#
                >
                    <Tabs>
                        <Tab active=true>"Overview"</Tab>
                        <Tab>"Analytics"</Tab>
                        <Tab>"Reports"</Tab>
                        <Tab disabled=true>"Disabled"</Tab>
                    </Tabs>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Variants"
                    code=r#"<Tabs variant={TabVariant::Bordered}>
    <Tab>"Bordered 1"</Tab>
    <Tab active=true>"Bordered 2"</Tab>
    <Tab>"Bordered 3"</Tab>
</Tabs>

<Tabs variant={TabVariant::Lifted}>
    <Tab>"Lifted 1"</Tab>
    <Tab active=true>"Lifted 2"</Tab>
    <Tab>"Lifted 3"</Tab>
</Tabs>

<Tabs variant={TabVariant::Boxed}>
    <Tab>"Boxed 1"</Tab>
    <Tab active=true>"Boxed 2"</Tab>
    <Tab>"Boxed 3"</Tab>
</Tabs>"#
                >
                    <div class="space-y-4">
                        <Tabs variant={TabVariant::Bordered}>
                            <Tab>"Bordered 1"</Tab>
                            <Tab active=true>"Bordered 2"</Tab>
                            <Tab>"Bordered 3"</Tab>
                        </Tabs>
                        <Tabs variant={TabVariant::Lifted}>
                            <Tab>"Lifted 1"</Tab>
                            <Tab active=true>"Lifted 2"</Tab>
                            <Tab>"Lifted 3"</Tab>
                        </Tabs>
                        <Tabs variant={TabVariant::Boxed}>
                            <Tab>"Boxed 1"</Tab>
                            <Tab active=true>"Boxed 2"</Tab>
                            <Tab>"Boxed 3"</Tab>
                        </Tabs>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r#"<Tabs size={Size::ExtraSmall}>
    <Tab active=true>"XS"</Tab>
    <Tab>"Tab"</Tab>
</Tabs>

<Tabs size={Size::Small}>
    <Tab active=true>"SM"</Tab>
    <Tab>"Tab"</Tab>
</Tabs>

<Tabs size={Size::Large}>
    <Tab active=true>"LG"</Tab>
    <Tab>"Tab"</Tab>
</Tabs>"#
                >
                    <div class="space-y-3">
                        <Tabs size={Size::ExtraSmall}>
                            <Tab active=true>"XS"</Tab>
                            <Tab>"Tab"</Tab>
                        </Tabs>
                        <Tabs size={Size::Small}>
                            <Tab active=true>"SM"</Tab>
                            <Tab>"Tab"</Tab>
                        </Tabs>
                        <Tabs size={Size::Large}>
                            <Tab active=true>"LG"</Tab>
                            <Tab>"Tab"</Tab>
                        </Tabs>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With content panel"
                    code=r#"<Tabs variant={TabVariant::Lifted}>
    <Tab active=true>"Profile"</Tab>
    <Tab>"Team"</Tab>
    <Tab>"Security"</Tab>
</Tabs>
<TabContent class="bg-base-100 border border-base-300 rounded-box p-4">
    <p class="font-medium">"Profile settings"</p>
    <p class="text-sm text-base-content/70 mt-1">
        "Use `TabContent` to style associated panel blocks with daisyUI classes."
    </p>
</TabContent>"#
                >
                    <div class="max-w-2xl">
                        <Tabs variant={TabVariant::Lifted}>
                            <Tab active=true>"Profile"</Tab>
                            <Tab>"Team"</Tab>
                            <Tab>"Security"</Tab>
                        </Tabs>
                        <TabContent class="bg-base-100 border border-base-300 rounded-box p-4">
                            <p class="font-medium">"Profile settings"</p>
                            <p class="text-sm text-base-content/70 mt-1">
                                "Use `TabContent` to style associated panel blocks with daisyUI classes."
                            </p>
                        </TabContent>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
