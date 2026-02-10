use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn BadgePage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Badge"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Badge>"Default"</Badge>
                    <Badge color={Color::Primary}>"Primary"</Badge>
                    <Badge color={Color::Secondary}>"Secondary"</Badge>
                    <Badge color={Color::Accent}>"Accent"</Badge>
                    <Badge color={Color::Info}>"Info"</Badge>
                    <Badge color={Color::Success}>"Success"</Badge>
                    <Badge color={Color::Warning}>"Warning"</Badge>
                    <Badge color={Color::Error}>"Error"</Badge>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-wrap items-center gap-2">
                    <Badge size={Size::ExtraSmall}>"XS"</Badge>
                    <Badge size={Size::Small}>"Small"</Badge>
                    <Badge size={Size::Medium}>"Medium"</Badge>
                    <Badge size={Size::Large}>"Large"</Badge>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Variants"</h2>
                <div class="flex flex-wrap gap-2">
                    <Badge variant={Variant::Outline} color={Color::Primary}>"Outline"</Badge>
                    <Badge variant={Variant::Soft} color={Color::Secondary}>"Soft"</Badge>
                    <Badge variant={Variant::Dash}>"Dash"</Badge>
                    <Badge variant={Variant::Ghost}>"Ghost"</Badge>
                </div>
            </section>
        </div>
    }
}
