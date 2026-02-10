use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn FileInputPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"File Input"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Small"</span>
                        <FileInput size={Size::ExtraSmall} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Small"</span>
                        <FileInput size={Size::Small} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Medium"</span>
                        <FileInput size={Size::Medium} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Large"</span>
                        <FileInput size={Size::Large} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Extra Large"</span>
                        <FileInput size={Size::ExtraLarge} />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Primary"</span>
                        <FileInput color={Color::Primary} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Secondary"</span>
                        <FileInput color={Color::Secondary} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Accent"</span>
                        <FileInput color={Color::Accent} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Info"</span>
                        <FileInput color={Color::Info} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Success"</span>
                        <FileInput color={Color::Success} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Warning"</span>
                        <FileInput color={Color::Warning} />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Error"</span>
                        <FileInput color={Color::Error} />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Variants"</h2>
                <div class="space-y-4">
                    <div>
                        <span class="text-sm text-base-content/70">"Bordered"</span>
                        <FileInput bordered=true />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Ghost"</span>
                        <FileInput ghost=true />
                    </div>
                    <div>
                        <span class="text-sm text-base-content/70">"Bordered + Primary Color"</span>
                        <FileInput bordered=true color={Color::Primary} />
                    </div>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Disabled"</h2>
                <FileInput disabled=true />
            </section>
        </div>
    }
}
