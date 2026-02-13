use crate::components::{ClassEntry, ClassTable, ComponentPreview};
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ArtboardPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Artboard"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A container that simulates mobile device screens for responsive design preview and testing."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-2xl font-bold">"Class Reference"</h2>
                <ClassTable entries=vec![
                    ClassEntry { name: "artboard", type_label: "base", description: "Base artboard class" },
                    ClassEntry { name: "artboard-horizontal", type_label: "modifier", description: "Horizontal orientation" },
                    ClassEntry { name: "phone-1", type_label: "size", description: "320x568" },
                    ClassEntry { name: "phone-2", type_label: "size", description: "375x667" },
                    ClassEntry { name: "phone-3", type_label: "size", description: "414x736" },
                    ClassEntry { name: "phone-4", type_label: "size", description: "375x812" },
                    ClassEntry { name: "phone-5", type_label: "size", description: "414x896" },
                    ClassEntry { name: "phone-6", type_label: "size", description: "320x1024" },
                ] />
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Artboard"
                    code=r#"<Artboard>
    <div class="p-4">"Content"</div>
</Artboard>"#
                >
                    <div class="flex flex-col items-center gap-2">
                        <Artboard>
                            <div class="p-4 h-32 flex items-center justify-center">
                                <p class="text-sm">"Default artboard without demo grid"</p>
                            </div>
                        </Artboard>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Phone Sizes"
                    code=r#"<Artboard size={ArtboardSize::Phone1} demo=true>
    <div class="p-4">"Phone 1 (320x568)"</div>
</Artboard>
<Artboard size={ArtboardSize::Phone2} demo=true>
    <div class="p-4">"Phone 2 (375x667)"</div>
</Artboard>
<Artboard size={ArtboardSize::Phone3} demo=true>
    <div class="p-4">"Phone 3 (414x736)"</div>
</Artboard>
<Artboard size={ArtboardSize::Phone4} demo=true>
    <div class="p-4">"Phone 4 (375x812)"</div>
</Artboard>
<Artboard size={ArtboardSize::Phone5} demo=true>
    <div class="p-4">"Phone 5 (414x896)"</div>
</Artboard>
<Artboard size={ArtboardSize::Phone6} demo=true>
    <div class="p-4">"Phone 6 (320x1024)"</div>
</Artboard>"#
                >
                    <div class="flex flex-wrap gap-8 items-start">
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone1} demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Phone 1"</p>
                                    <p class="text-xs text-base-content/70">"320x568"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Phone 1"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone2} demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Phone 2"</p>
                                    <p class="text-xs text-base-content/70">"375x667"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Phone 2"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone3} demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Phone 3"</p>
                                    <p class="text-xs text-base-content/70">"414x736"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Phone 3"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone4} demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Phone 4"</p>
                                    <p class="text-xs text-base-content/70">"375x812"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Phone 4"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone5} demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Phone 5"</p>
                                    <p class="text-xs text-base-content/70">"414x896"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Phone 5"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone6} demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Phone 6"</p>
                                    <p class="text-xs text-base-content/70">"320x1024"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Phone 6"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Horizontal Orientation"
                    code=r#"<Artboard size={ArtboardSize::Phone4} horizontal=true demo=true>
    <div class="p-4">"Landscape"</div>
</Artboard>"#
                >
                    <div class="flex flex-wrap gap-8">
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone4} horizontal=true demo=true>
                                <div class="p-4">
                                    <p class="text-sm">"Landscape"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Horizontal"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="With Content"
                    code=r#"<Artboard size={ArtboardSize::Phone4} demo=true>
    <div class="h-full flex flex-col">
        <div class="bg-primary text-primary-content p-4">
            <h3 class="font-bold">"My App"</h3>
        </div>
        <div class="flex-1 p-4 space-y-4">
            <div class="bg-base-200 p-3 rounded">
                <p class="text-sm">"Welcome to the app!"</p>
            </div>
            <Button size={Size::Small} color={Color::Primary} class="w-full">"Get Started"</Button>
        </div>
    </div>
</Artboard>"#
                >
                    <Artboard size={ArtboardSize::Phone4} demo=true>
                        <div class="h-full flex flex-col">
                            <div class="bg-primary text-primary-content p-4">
                                <h3 class="font-bold">"My App"</h3>
                            </div>
                            <div class="flex-1 p-4 space-y-4">
                                <div class="bg-base-200 p-3 rounded">
                                    <p class="text-sm">"Welcome to the app!"</p>
                                </div>
                                <Button size={Size::Small} color={Color::Primary} class="w-full">"Get Started"</Button>
                            </div>
                        </div>
                    </Artboard>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Demo Mode"
                    code=r#"<Artboard demo=true>
    <div class="p-4">"Demo grid background"</div>
</Artboard>
<Artboard size={ArtboardSize::Phone4} demo=true>
    <div class="p-4">"Phone 4 with demo grid"</div>
</Artboard>"#
                >
                    <div class="flex flex-wrap gap-8">
                        <div class="flex flex-col items-center gap-2">
                            <Artboard demo=true>
                                <div class="p-4 h-32 flex items-center justify-center">
                                    <p class="text-sm">"Demo grid background"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"artboard-demo enabled"</span>
                        </div>
                        <div class="flex flex-col items-center gap-2">
                            <Artboard size={ArtboardSize::Phone4} demo=true>
                                <div class="p-4 h-full flex items-center justify-center">
                                    <p class="text-sm">"Phone 4 with demo grid"</p>
                                </div>
                            </Artboard>
                            <span class="text-xs text-base-content/70">"Demo + Phone 4"</span>
                        </div>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Phone Mockup Pattern"
                    code=r#"<Artboard size={ArtboardSize::Phone5} demo=true>
    <div class="h-full bg-base-200 p-4">
        <div class="bg-primary text-primary-content p-3 rounded-t-lg">
            <div class="flex justify-between items-center">
                <span class="text-sm font-medium">"9:41"</span>
            </div>
        </div>
        <div class="p-4 space-y-3">
            <h4 class="font-semibold">"App Interface"</h4>
            <div class="space-y-2">
                <div class="bg-base-100 p-2 rounded">
                    <p class="text-xs">"Welcome to the app!"</p>
                </div>
            </div>
        </div>
    </div>
</Artboard>"#
                >
                    <Artboard size={ArtboardSize::Phone5} demo=true>
                        <div class="h-full bg-base-200 p-4">
                            <div class="bg-primary text-primary-content p-3 rounded-t-lg">
                                <div class="flex justify-between items-center">
                                    <span class="text-sm font-medium">"9:41"</span>
                                    <div class="flex gap-1">
                                        <div class="w-1 h-1 bg-current rounded-full"></div>
                                        <div class="w-1 h-1 bg-current rounded-full"></div>
                                        <div class="w-1 h-1 bg-current rounded-full"></div>
                                    </div>
                                </div>
                            </div>
                            <div class="p-4 space-y-3">
                                <h4 class="font-semibold">"App Interface"</h4>
                                <div class="space-y-2">
                                    <div class="bg-base-100 p-2 rounded">
                                        <p class="text-xs">"Welcome to the app!"</p>
                                    </div>
                                    <div class="bg-base-100 p-2 rounded">
                                        <p class="text-xs">"Your content here"</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Artboard>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Without Demo Grid"
                    code=r#"<Artboard size={ArtboardSize::Phone3}>
    <div class="p-4 h-full flex items-center justify-center">
        <p>"Clean artboard without grid background"</p>
    </div>
</Artboard>"#
                >
                    <Artboard size={ArtboardSize::Phone3}>
                        <div class="p-4 h-full flex items-center justify-center">
                            <p>"Clean artboard without grid background"</p>
                        </div>
                    </Artboard>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Custom Classes"
                    code=r#"<Artboard size={ArtboardSize::Phone4} demo=true class="border-2 border-primary rounded-lg shadow-xl">
    <div class="p-4">"Artboard with custom border and shadow"</div>
</Artboard>"#
                >
                    <div class="flex flex-col items-center gap-2">
                        <Artboard
                            size={ArtboardSize::Phone4}
                            demo=true
                            class="border-2 border-primary rounded-lg shadow-xl"
                        >
                            <div class="p-4 h-full flex items-center justify-center">
                                <p class="text-sm">"Artboard with custom border and shadow"</p>
                            </div>
                        </Artboard>
                        <span class="text-xs text-base-content/70">"Custom classes added via class prop"</span>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
