use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TabPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Tabs"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Default Tabs"</h2>
                <Tabs>
                    <Tab active=true>"Tab 1"</Tab>
                    <Tab>"Tab 2"</Tab>
                    <Tab>"Tab 3"</Tab>
                </Tabs>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <Tabs size={Size::ExtraSmall}>
                            <Tab active=true>"XS Tab"</Tab>
                            <Tab>"XS Tab"</Tab>
                        </Tabs>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <Tabs size={Size::Small}>
                            <Tab active=true>"Small Tab"</Tab>
                            <Tab>"Small Tab"</Tab>
                        </Tabs>
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <Tabs size={Size::Large}>
                            <Tab active=true>"Large Tab"</Tab>
                            <Tab>"Large Tab"</Tab>
                        </Tabs>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Lift Variant"</h2>
                <Tabs variant={TabVariant::Lift}>
                    <Tab active=true>"Home"</Tab>
                    <Tab>"Profile"</Tab>
                    <Tab>"Settings"</Tab>
                </Tabs>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Box Variant"</h2>
                <Tabs variant={TabVariant::Box}>
                    <Tab active=true>"Tab 1"</Tab>
                    <Tab>"Tab 2"</Tab>
                    <Tab>"Tab 3"</Tab>
                </Tabs>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Border Variant"</h2>
                <Tabs variant={TabVariant::Border}>
                    <Tab active=true>"Tab 1"</Tab>
                    <Tab>"Tab 2"</Tab>
                    <Tab>"Tab 3"</Tab>
                </Tabs>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Bottom Position"</h2>
                <div class="border rounded-lg p-4">
                    <div class="h-24 flex items-end">
                        <Tabs position={TabPosition::Bottom} variant={TabVariant::Lift}>
                            <Tab active=true>"Tab 1"</Tab>
                            <Tab>"Tab 2"</Tab>
                            <Tab>"Tab 3"</Tab>
                        </Tabs>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled Tab"</h2>
                <Tabs>
                    <Tab active=true>"Active"</Tab>
                    <Tab>"Normal"</Tab>
                    <Tab disabled=true>"Disabled"</Tab>
                </Tabs>
            </section>
        </div>
    }
}
