use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Button"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Complete daisyUI button variants with practical combinations and accessibility patterns."
                </p>
            </header>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button>"Default"</Button>
                    <Button color={Color::Neutral}>"Neutral"</Button>
                    <Button color={Color::Primary}>"Primary"</Button>
                    <Button color={Color::Secondary}>"Secondary"</Button>
                    <Button color={Color::Accent}>"Accent"</Button>
                    <Button color={Color::Info}>"Info"</Button>
                    <Button color={Color::Success}>"Success"</Button>
                    <Button color={Color::Warning}>"Warning"</Button>
                    <Button color={Color::Error}>"Error"</Button>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Styles"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button variant={Variant::Solid} color={Color::Primary}>"Solid"</Button>
                    <Button variant={Variant::Outline} color={Color::Primary}>"Outline"</Button>
                    <Button variant={Variant::Ghost}>"Ghost"</Button>
                    <Button variant={Variant::Link}>"Link"</Button>
                    <Button variant={Variant::Soft} color={Color::Primary}>"Soft"</Button>
                    <Button variant={Variant::Dash}>"Dash"</Button>
                    <Button glass=true color={Color::Secondary}>"Glass"</Button>
                    <Button no_animation=true color={Color::Accent}>"No animation"</Button>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Sizes and shape"</h2>
                <div class="flex flex-wrap items-center gap-2">
                    <Button size={Size::ExtraSmall}>"XS"</Button>
                    <Button size={Size::Small}>"Small"</Button>
                    <Button size={Size::Medium}>"Medium"</Button>
                    <Button size={Size::Large}>"Large"</Button>
                    <Button size={Size::ExtraLarge}>"XL"</Button>
                    <Button square=true color={Color::Primary}>
                        "■"
                    </Button>
                    <Button circle=true color={Color::Secondary}>
                        "●"
                    </Button>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"State"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button state={State::Active}>"Active"</Button>
                    <Button state={State::Loading}>"Loading"</Button>
                    <Button disabled=true>"Disabled"</Button>
                    <Button color={Color::Primary}>
                        <span class="loading loading-spinner loading-xs mr-1"></span>
                        "With spinner"
                    </Button>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-xl font-semibold">"Layout patterns"</h2>
                <div class="space-y-3 max-w-xl">
                    <Button wide=true color={Color::Primary}>"Wide Button"</Button>
                    <Button block=true color={Color::Secondary}>"Block Button"</Button>
                    <div class="join">
                        <Button class="join-item" variant={Variant::Outline}>"Previous"</Button>
                        <Button class="join-item" color={Color::Primary}>"Current"</Button>
                        <Button class="join-item" variant={Variant::Outline}>"Next"</Button>
                    </div>
                </div>
            </section>
        </div>
    }
}
