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
                <h2 class="text-xl font-semibold">"Default"</h2>
                <Tabs>
                    <Tab active=true>"Overview"</Tab>
                    <Tab>"Analytics"</Tab>
                    <Tab>"Reports"</Tab>
                    <Tab disabled=true>"Disabled"</Tab>
                </Tabs>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Variants"</h2>
                <div class="space-y-4">
                    <Tabs variant=TabVariant::Bordered>
                        <Tab>"Bordered 1"</Tab>
                        <Tab active=true>"Bordered 2"</Tab>
                        <Tab>"Bordered 3"</Tab>
                    </Tabs>
                    <Tabs variant=TabVariant::Lifted>
                        <Tab>"Lifted 1"</Tab>
                        <Tab active=true>"Lifted 2"</Tab>
                        <Tab>"Lifted 3"</Tab>
                    </Tabs>
                    <Tabs variant=TabVariant::Boxed>
                        <Tab>"Boxed 1"</Tab>
                        <Tab active=true>"Boxed 2"</Tab>
                        <Tab>"Boxed 3"</Tab>
                    </Tabs>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes"</h2>
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
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"With content panel"</h2>
                <div class="max-w-2xl">
                    <Tabs variant=TabVariant::Lifted>
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
            </section>
        </div>
    }
}
