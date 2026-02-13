use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn RatingPage() -> impl IntoView {
    let (selected, set_selected) = signal("3".to_string());
    let star_1 = Signal::derive(move || selected.get() == "1");
    let star_2 = Signal::derive(move || selected.get() == "2");
    let star_3 = Signal::derive(move || selected.get() == "3");
    let star_4 = Signal::derive(move || selected.get() == "4");
    let star_5 = Signal::derive(move || selected.get() == "5");

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Rating"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Star and shape-based ratings with size and half-step support."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic"
                    code=r##"<Rating>
    <RatingItem name="rating-basic" value="1" />
    <RatingItem name="rating-basic" value="2" />
    <RatingItem name="rating-basic" value="3" checked=true />
    <RatingItem name="rating-basic" value="4" />
    <RatingItem name="rating-basic" value="5" />
</Rating>"##
                >
                    <Rating>
                        <RatingItem name="rating-basic" value="1" />
                        <RatingItem name="rating-basic" value="2" />
                        <RatingItem name="rating-basic" value="3" checked=true />
                        <RatingItem name="rating-basic" value="4" />
                        <RatingItem name="rating-basic" value="5" />
                    </Rating>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Mask shapes"
                    code=r##"<Rating>
    <RatingItem name="rating-shape-star" value="1" mask=RatingMask::Star2 checked=true />
    <RatingItem name="rating-shape-star" value="2" mask=RatingMask::Star2 />
    <RatingItem name="rating-shape-star" value="3" mask=RatingMask::Star2 />
    <RatingItem name="rating-shape-star" value="4" mask=RatingMask::Star2 />
</Rating>

<Rating>
    <RatingItem name="rating-shape-heart" value="1" mask=RatingMask::Heart checked=true />
    <RatingItem name="rating-shape-heart" value="2" mask=RatingMask::Heart />
    <RatingItem name="rating-shape-heart" value="3" mask=RatingMask::Heart />
    <RatingItem name="rating-shape-heart" value="4" mask=RatingMask::Heart />
</Rating>

<Rating>
    <RatingItem name="rating-shape-heart-fill" value="1" mask=RatingMask::HeartFill checked=true />
    <RatingItem name="rating-shape-heart-fill" value="2" mask=RatingMask::HeartFill />
    <RatingItem name="rating-shape-heart-fill" value="3" mask=RatingMask::HeartFill />
    <RatingItem name="rating-shape-heart-fill" value="4" mask=RatingMask::HeartFill />
</Rating>"##
                >
                    <div class="space-y-3">
                        <Rating>
                            <RatingItem name="rating-shape-star" value="1" mask=RatingMask::Star2 checked=true />
                            <RatingItem name="rating-shape-star" value="2" mask=RatingMask::Star2 />
                            <RatingItem name="rating-shape-star" value="3" mask=RatingMask::Star2 />
                            <RatingItem name="rating-shape-star" value="4" mask=RatingMask::Star2 />
                        </Rating>
                        <Rating>
                            <RatingItem name="rating-shape-heart" value="1" mask=RatingMask::Heart checked=true />
                            <RatingItem name="rating-shape-heart" value="2" mask=RatingMask::Heart />
                            <RatingItem name="rating-shape-heart" value="3" mask=RatingMask::Heart />
                            <RatingItem name="rating-shape-heart" value="4" mask=RatingMask::Heart />
                        </Rating>
                        <Rating>
                            <RatingItem
                                name="rating-shape-heart-fill"
                                value="1"
                                mask=RatingMask::HeartFill
                                checked=true
                            />
                            <RatingItem name="rating-shape-heart-fill" value="2" mask=RatingMask::HeartFill />
                            <RatingItem name="rating-shape-heart-fill" value="3" mask=RatingMask::HeartFill />
                            <RatingItem name="rating-shape-heart-fill" value="4" mask=RatingMask::HeartFill />
                        </Rating>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Rating size=Size::ExtraSmall>
    <RatingItem name="rating-xs" value="1" checked=true />
    <RatingItem name="rating-xs" value="2" />
    <RatingItem name="rating-xs" value="3" />
</Rating>

<Rating size=Size::Small>
    <RatingItem name="rating-sm" value="1" checked=true />
    <RatingItem name="rating-sm" value="2" />
    <RatingItem name="rating-sm" value="3" />
</Rating>

<Rating size=Size::Medium>
    <RatingItem name="rating-md" value="1" checked=true />
    <RatingItem name="rating-md" value="2" />
    <RatingItem name="rating-md" value="3" />
</Rating>

<Rating size=Size::Large>
    <RatingItem name="rating-lg" value="1" checked=true />
    <RatingItem name="rating-lg" value="2" />
    <RatingItem name="rating-lg" value="3" />
</Rating>

<Rating size=Size::ExtraLarge>
    <RatingItem name="rating-xl" value="1" checked=true />
    <RatingItem name="rating-xl" value="2" />
    <RatingItem name="rating-xl" value="3" />
</Rating>"##
                >
                    <div class="space-y-3">
                        <Rating size=Size::ExtraSmall>
                            <RatingItem name="rating-xs" value="1" checked=true />
                            <RatingItem name="rating-xs" value="2" />
                            <RatingItem name="rating-xs" value="3" />
                        </Rating>
                        <Rating size=Size::Small>
                            <RatingItem name="rating-sm" value="1" checked=true />
                            <RatingItem name="rating-sm" value="2" />
                            <RatingItem name="rating-sm" value="3" />
                        </Rating>
                        <Rating size=Size::Medium>
                            <RatingItem name="rating-md" value="1" checked=true />
                            <RatingItem name="rating-md" value="2" />
                            <RatingItem name="rating-md" value="3" />
                        </Rating>
                        <Rating size=Size::Large>
                            <RatingItem name="rating-lg" value="1" checked=true />
                            <RatingItem name="rating-lg" value="2" />
                            <RatingItem name="rating-lg" value="3" />
                        </Rating>
                        <Rating size=Size::ExtraLarge>
                            <RatingItem name="rating-xl" value="1" checked=true />
                            <RatingItem name="rating-xl" value="2" />
                            <RatingItem name="rating-xl" value="3" />
                        </Rating>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Half rating"
                    code=r##"<Rating half=true>
    <RatingItem name="rating-half" value="0.5" half=RatingHalf::First />
    <RatingItem name="rating-half" value="1.0" half=RatingHalf::Second checked=true />
    <RatingItem name="rating-half" value="1.5" half=RatingHalf::First />
    <RatingItem name="rating-half" value="2.0" half=RatingHalf::Second />
    <RatingItem name="rating-half" value="2.5" half=RatingHalf::First />
    <RatingItem name="rating-half" value="3.0" half=RatingHalf::Second />
</Rating>"##
                >
                    <Rating half=true>
                        <RatingItem name="rating-half" value="0.5" half=RatingHalf::First />
                        <RatingItem name="rating-half" value="1.0" half=RatingHalf::Second checked=true />
                        <RatingItem name="rating-half" value="1.5" half=RatingHalf::First />
                        <RatingItem name="rating-half" value="2.0" half=RatingHalf::Second />
                        <RatingItem name="rating-half" value="2.5" half=RatingHalf::First />
                        <RatingItem name="rating-half" value="3.0" half=RatingHalf::Second />
                    </Rating>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Disabled"
                    code=r##"<Rating>
    <RatingItem name="rating-disabled" value="1" disabled=true />
    <RatingItem name="rating-disabled" value="2" checked=true disabled=true />
    <RatingItem name="rating-disabled" value="3" disabled=true />
    <RatingItem name="rating-disabled" value="4" disabled=true />
    <RatingItem name="rating-disabled" value="5" disabled=true />
</Rating>"##
                >
                    <Rating>
                        <RatingItem name="rating-disabled" value="1" disabled=true />
                        <RatingItem name="rating-disabled" value="2" checked=true disabled=true />
                        <RatingItem name="rating-disabled" value="3" disabled=true />
                        <RatingItem name="rating-disabled" value="4" disabled=true />
                        <RatingItem name="rating-disabled" value="5" disabled=true />
                    </Rating>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive example"
                    code=r##"let (selected, set_selected) = signal("3".to_string());
let star_1 = Signal::derive(move || selected.get() == "1");
let star_2 = Signal::derive(move || selected.get() == "2");
let star_3 = Signal::derive(move || selected.get() == "3");
let star_4 = Signal::derive(move || selected.get() == "4");
let star_5 = Signal::derive(move || selected.get() == "5");

view! {
    <Rating>
        <RatingItem
            name="rating-reactive"
            value="1"
            checked=star_1
            on:change=move |_| set_selected.set("1".to_string())
        />
        <RatingItem
            name="rating-reactive"
            value="2"
            checked=star_2
            on:change=move |_| set_selected.set("2".to_string())
        />
        <RatingItem
            name="rating-reactive"
            value="3"
            checked=star_3
            on:change=move |_| set_selected.set("3".to_string())
        />
        <RatingItem
            name="rating-reactive"
            value="4"
            checked=star_4
            on:change=move |_| set_selected.set("4".to_string())
        />
        <RatingItem
            name="rating-reactive"
            value="5"
            checked=star_5
            on:change=move |_| set_selected.set("5".to_string())
        />
    </Rating>
    <p class="text-sm text-base-content/70">
        {move || format!("Selected rating: {}", selected.get())}
    </p>
}"##
                >
                    <div class="space-y-3">
                        <Rating>
                            <RatingItem
                                name="rating-reactive"
                                value="1"
                                checked=star_1
                                on:change=move |_| set_selected.set("1".to_string())
                            />
                            <RatingItem
                                name="rating-reactive"
                                value="2"
                                checked=star_2
                                on:change=move |_| set_selected.set("2".to_string())
                            />
                            <RatingItem
                                name="rating-reactive"
                                value="3"
                                checked=star_3
                                on:change=move |_| set_selected.set("3".to_string())
                            />
                            <RatingItem
                                name="rating-reactive"
                                value="4"
                                checked=star_4
                                on:change=move |_| set_selected.set("4".to_string())
                            />
                            <RatingItem
                                name="rating-reactive"
                                value="5"
                                checked=star_5
                                on:change=move |_| set_selected.set("5".to_string())
                            />
                        </Rating>
                        <p class="text-sm text-base-content/70">{move || format!("Selected rating: {}", selected.get())}</p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
