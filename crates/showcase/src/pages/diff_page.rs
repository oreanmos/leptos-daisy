use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn DiffPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Diff"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Before/After Comparison"</h2>
                <Diff class="aspect-video max-w-2xl">
                    <DiffItem1>
                        <img
                            src="https://picsum.photos/seed/before/800/450?grayscale"
                            alt="Before"
                            class="w-full h-full object-cover"
                        />
                    </DiffItem1>
                    <DiffItem2>
                        <img
                            src="https://picsum.photos/seed/after/800/450"
                            alt="After"
                            class="w-full h-full object-cover"
                        />
                    </DiffItem2>
                    <DiffResizer />
                </Diff>
                <p class="text-sm text-base-content/70 mt-2">"Drag the slider to compare before and after"</p>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Text Comparison"</h2>
                <Diff class="aspect-[2/1] max-w-xl">
                    <DiffItem1>
                        <div class="w-full h-full bg-base-300 flex items-center justify-center p-8">
                            <div class="text-center">
                                <h3 class="text-xl font-bold mb-2">"Old Version"</h3>
                                <p>"Lorem ipsum dolor sit amet"</p>
                                <p>"consectetur adipiscing elit"</p>
                                <p>"sed do eiusmod tempor"</p>
                            </div>
                        </div>
                    </DiffItem1>
                    <DiffItem2>
                        <div class="w-full h-full bg-primary flex items-center justify-center p-8 text-primary-content">
                            <div class="text-center">
                                <h3 class="text-xl font-bold mb-2">"New Version"</h3>
                                <p>"Lorem ipsum dolor sit amet"</p>
                                <p>"consectetur adipiscing elit"</p>
                                <p>"sed do eiusmod tempor incididunt"</p>
                                <p>"ut labore et dolore magna"</p>
                            </div>
                        </div>
                    </DiffItem2>
                    <DiffResizer />
                </Diff>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Color Comparison"</h2>
                <Diff class="aspect-video max-w-lg">
                    <DiffItem1>
                        <div class="w-full h-full bg-gradient-to-r from-blue-500 to-purple-500 flex items-center justify-center">
                            <span class="text-white text-2xl font-bold">"Gradient A"</span>
                        </div>
                    </DiffItem1>
                    <DiffItem2>
                        <div class="w-full h-full bg-gradient-to-r from-red-500 to-orange-500 flex items-center justify-center">
                            <span class="text-white text-2xl font-bold">"Gradient B"</span>
                        </div>
                    </DiffItem2>
                    <DiffResizer />
                </Diff>
            </section>
        </div>
    }
}
