use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn MockupBrowserPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Mockup Browser"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Browser window mockup for displaying web content previews."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "mockup-browser", type_label: "base", description: "Base class for browser mockup container" },
                    ClassEntry { name: "mockup-browser-toolbar", type_label: "base part", description: "Toolbar area with URL bar" },
                ] />
            </section>

            <section>
                <ComponentPreview
                    title="Basic Browser"
                    code=r#"<MockupBrowser>
    <div class="p-8 bg-base-200">
        <h3 class="text-2xl font-bold">"Hello World!"</h3>
        <p>"This is content inside a browser mockup."</p>
    </div>
</MockupBrowser>"#
                >
                    <MockupBrowser>
                        <div class="p-8 bg-base-200">
                            <h3 class="text-2xl font-bold mb-4">"Hello World!"</h3>
                            <p>"This is content inside a browser mockup."</p>
                        </div>
                    </MockupBrowser>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With URL"
                    code=r#"<MockupBrowser url="https://daisyui.com">
    <div class="p-8 bg-base-200">
        <div class="hero bg-base-100 rounded-lg">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold">"daisyUI"</h1>
                    <p class="py-6">"The most popular component library for Tailwind CSS."</p>
                    <Button color={Color::Primary}>"Get Started"</Button>
                </div>
            </div>
        </div>
    </div>
</MockupBrowser>"#
                >
                    <MockupBrowser url="https://daisyui.com">
                        <div class="p-8 bg-base-200">
                            <div class="hero bg-base-100 rounded-lg">
                                <div class="hero-content text-center">
                                    <div class="max-w-md">
                                        <h1 class="text-5xl font-bold">"daisyUI"</h1>
                                        <p class="py-6">"The most popular component library for Tailwind CSS."</p>
                                        <Button color={Color::Primary}>"Get Started"</Button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </MockupBrowser>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Different Content"
                    code=r#"<MockupBrowser url="https://example.com/dashboard">
    <div class="p-4 bg-base-200">
        <div class="grid grid-cols-3 gap-4">
            <Card class="bg-base-100">
                <CardBody>
                    <Stat>
                        <StatTitle>"Users"</StatTitle>
                        <StatValue>"12,345"</StatValue>
                    </Stat>
                </CardBody>
            </Card>
            // ... more cards
        </div>
    </div>
</MockupBrowser>"#
                >
                    <div class="space-y-4">
                        <MockupBrowser url="https://example.com/dashboard">
                            <div class="p-4 bg-base-200">
                                <div class="grid grid-cols-3 gap-4">
                                    <Card class="bg-base-100">
                                        <CardBody>
                                            <Stat>
                                                <StatTitle>"Users"</StatTitle>
                                                <StatValue>"12,345"</StatValue>
                                            </Stat>
                                        </CardBody>
                                    </Card>
                                    <Card class="bg-base-100">
                                        <CardBody>
                                            <Stat>
                                                <StatTitle>"Revenue"</StatTitle>
                                                <StatValue>"$45K"</StatValue>
                                            </Stat>
                                        </CardBody>
                                    </Card>
                                    <Card class="bg-base-100">
                                        <CardBody>
                                            <Stat>
                                                <StatTitle>"Orders"</StatTitle>
                                                <StatValue>"892"</StatValue>
                                            </Stat>
                                        </CardBody>
                                    </Card>
                                </div>
                            </div>
                        </MockupBrowser>

                        <MockupBrowser url="https://example.com/login">
                            <div class="p-8 bg-base-200 flex justify-center">
                                <Card class="w-96 bg-base-100">
                                    <CardBody>
                                        <h2 class="card-title justify-center mb-4">"Login"</h2>
                                        <div class="space-y-4">
                                            <div>
                                                <Label>"Email"</Label>
                                                <Input placeholder="email@example.com" />
                                            </div>
                                            <div>
                                                <Label>"Password"</Label>
                                                <Input input_type="password" placeholder="••••••••" />
                                            </div>
                                            <Button color={Color::Primary} class="w-full">"Sign In"</Button>
                                        </div>
                                    </CardBody>
                                </Card>
                            </div>
                        </MockupBrowser>
                    </div>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="With Image Content"
                    code=r#"<MockupBrowser url="https://picsum.photos">
    <div class="bg-base-200">
        <img src="https://picsum.photos/800/400" alt="Sample" class="w-full h-64 object-cover" />
        <div class="p-4">
            <h3 class="font-bold">"Beautiful Landscape"</h3>
            <p class="text-sm">"A stunning view captured in high resolution."</p>
        </div>
    </div>
</MockupBrowser>"#
                >
                    <MockupBrowser url="https://picsum.photos">
                        <div class="bg-base-200">
                            <img src="https://picsum.photos/800/400" alt="Sample" class="w-full h-64 object-cover" />
                            <div class="p-4">
                                <h3 class="font-bold">"Beautiful Landscape"</h3>
                                <p class="text-sm text-base-content/70">"A stunning view captured in high resolution."</p>
                            </div>
                        </div>
                    </MockupBrowser>
                </ComponentPreview>
            </section>

            <section>
                <ComponentPreview
                    title="Responsive Sizes"
                    code=r#"<MockupBrowser url="https://mobile.example.com" class="max-w-sm mx-auto">
    <div class="p-4 bg-base-200">
        <p class="text-center">"Mobile viewport"</p>
    </div>
</MockupBrowser>
<MockupBrowser url="https://tablet.example.com" class="max-w-2xl mx-auto">
    <div class="p-6 bg-base-200">
        <p class="text-center">"Tablet viewport"</p>
    </div>
</MockupBrowser>
<MockupBrowser url="https://desktop.example.com">
    <div class="p-8 bg-base-200">
        <p class="text-center">"Desktop viewport"</p>
    </div>
</MockupBrowser>"#
                >
                    <div class="space-y-4">
                        <MockupBrowser url="https://mobile.example.com" class="max-w-sm mx-auto">
                            <div class="p-4 bg-base-200">
                                <p class="text-center">"Mobile viewport"</p>
                            </div>
                        </MockupBrowser>
                        <MockupBrowser url="https://tablet.example.com" class="max-w-2xl mx-auto">
                            <div class="p-6 bg-base-200">
                                <p class="text-center">"Tablet viewport"</p>
                            </div>
                        </MockupBrowser>
                        <MockupBrowser url="https://desktop.example.com">
                            <div class="p-8 bg-base-200">
                                <p class="text-center">"Desktop viewport"</p>
                            </div>
                        </MockupBrowser>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
