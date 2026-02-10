use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ArtboardPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Artboard"</h1>

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
                <h2 class="text-xl font-semibold mb-4">"Without Demo Grid"</h2>
                <Artboard size={ArtboardSize::Phone3}>
                    <div class="p-4 h-full flex items-center justify-center">
                        <p>"Clean artboard without grid background"</p>
                    </div>
                </Artboard>
            </section>
        </div>
    }
}
