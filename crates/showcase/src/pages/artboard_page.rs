use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ArtboardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Artboard"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Basic Artboard"</h2>
                <div class="flex flex-col items-center gap-2">
                    <Artboard>
                        <div class="p-4 h-32 flex items-center justify-center">
                            <p class="text-sm">"Default artboard without demo grid"</p>
                        </div>
                    </Artboard>
                    <span class="text-xs text-base-content/70">"Basic example with default size"</span>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Phone Sizes"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Horizontal Orientation"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"With Content"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Demo Mode Showcase"</h2>
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
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Usage Patterns"</h2>
                <div class="space-y-6">
                    <div>
                        <h3 class="text-lg font-medium mb-2">"Phone Mockup"</h3>
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
                    </div>

                    <div>
                        <h3 class="text-lg font-medium mb-2">"Screenshot Preview"</h3>
                        <Artboard size={ArtboardSize::Phone3} demo=true>
                            <img
                                src="https://via.placeholder.com/414x736/ffffff/000000?text=App+Screenshot"
                                alt="App screenshot preview"
                                class="w-full h-full object-cover"
                            />
                        </Artboard>
                    </div>

                    <div>
                        <h3 class="text-lg font-medium mb-2">"Mobile Design Showcase"</h3>
                        <Artboard size={ArtboardSize::Phone6} horizontal=true demo=true>
                            <div class="h-full bg-gradient-to-r from-primary to-secondary flex items-center justify-center p-4">
                                <div class="text-center text-primary-content">
                                    <h4 class="font-bold mb-2">"Beautiful Design"</h4>
                                    <p class="text-sm">"Showcase your mobile app design"</p>
                                </div>
                            </div>
                        </Artboard>
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Without Demo Grid"</h2>
                <Artboard size={ArtboardSize::Phone3}>
                    <div class="p-4 h-full flex items-center justify-center">
                        <p>"Clean artboard without grid background"</p>
                    </div>
                </Artboard>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Custom Classes"</h2>
                <div>
                    <h3 class="text-lg font-medium mb-2">"Additional Custom Classes"</h3>
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
            </section>
        </div>
    }
}
