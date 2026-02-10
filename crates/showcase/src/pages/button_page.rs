use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Button"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button>"Default"</Button>
                    <Button color={Color::Primary}>"Primary"</Button>
                    <Button color={Color::Secondary}>"Secondary"</Button>
                    <Button color={Color::Accent}>"Accent"</Button>
                    <Button color={Color::Info}>"Info"</Button>
                    <Button color={Color::Success}>"Success"</Button>
                    <Button color={Color::Warning}>"Warning"</Button>
                    <Button color={Color::Error}>"Error"</Button>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Sizes"</h2>
                <div class="flex flex-wrap items-center gap-2">
                    <Button size={Size::ExtraSmall}>"XS"</Button>
                    <Button size={Size::Small}>"Small"</Button>
                    <Button size={Size::Medium}>"Medium"</Button>
                    <Button size={Size::Large}>"Large"</Button>
                    <Button size={Size::ExtraLarge}>"XL"</Button>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Variants"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button variant={Variant::Solid} color={Color::Primary}>"Solid"</Button>
                    <Button variant={Variant::Outline} color={Color::Primary}>"Outline"</Button>
                    <Button variant={Variant::Ghost}>"Ghost"</Button>
                    <Button variant={Variant::Link}>"Link"</Button>
                    <Button variant={Variant::Soft} color={Color::Primary}>"Soft"</Button>
                    <Button variant={Variant::Dash}>"Dash"</Button>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"States"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button state={State::Active}>"Active"</Button>
                    <Button disabled=true>"Disabled"</Button>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Wide & Block"</h2>
                <div class="space-y-2">
                    <Button wide=true color={Color::Primary}>"Wide Button"</Button>
                    <Button class="btn-block" color={Color::Secondary}>"Block Button"</Button>
                </div>
            </section>
        </div>
    }
}
