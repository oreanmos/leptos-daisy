use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DividerPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Divider"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Divider"</h2>
                <div class="space-y-4">
                    <p>"Content above the divider"</p>
                    <Divider />
                    <p>"Content below the divider"</p>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical Divider"</h2>
                <div class="flex h-24">
                    <div class="flex-1 flex items-center justify-center">"Left content"</div>
                    <Divider orientation={DividerOrientation::Vertical} />
                    <div class="flex-1 flex items-center justify-center">"Right content"</div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Divider with Text"</h2>
                <div class="space-y-4">
                    <p>"Some content here"</p>
                    <Divider text="OR".to_string() />
                    <p>"More content here"</p>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Divider Colors"</h2>
                <div class="space-y-4">
                    <Divider color={DividerColor::Primary} text="Primary".to_string() />
                    <Divider color={DividerColor::Secondary} text="Secondary".to_string() />
                    <Divider color={DividerColor::Accent} text="Accent".to_string() />
                    <Divider color={DividerColor::Neutral} text="Neutral".to_string() />
                    <Divider color={DividerColor::Info} text="Info".to_string() />
                    <Divider color={DividerColor::Success} text="Success".to_string() />
                    <Divider color={DividerColor::Warning} text="Warning".to_string() />
                    <Divider color={DividerColor::Error} text="Error".to_string() />
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Vertical with Colors"</h2>
                <div class="flex h-24 gap-4">
                    <div class="flex-1 flex items-center justify-center">"Content"</div>
                    <Divider orientation={DividerOrientation::Vertical} color={DividerColor::Primary} />
                    <div class="flex-1 flex items-center justify-center">"Content"</div>
                    <Divider orientation={DividerOrientation::Vertical} color={DividerColor::Secondary} />
                    <div class="flex-1 flex items-center justify-center">"Content"</div>
                    <Divider orientation={DividerOrientation::Vertical} color={DividerColor::Accent} />
                    <div class="flex-1 flex items-center justify-center">"Content"</div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"In Context"</h2>
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
            </section>
        </div>
    }
}
