use crate::components::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn LabelButtonPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Label Button"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "A "<code>"<label>"</code>" element styled as a daisyUI button. "
                    "Useful for drawer toggles and file upload triggers."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Basic Usage"
                    code=r##"<LabelButton for_id="my-checkbox" color=Color::Primary>"Toggle"</LabelButton>"##
                >
                    <div class="flex flex-wrap gap-2 items-center">
                        <input type="checkbox" id="label-btn-demo" class="checkbox" />
                        <LabelButton for_id="label-btn-demo" color=Color::Primary>"Toggle Checkbox"</LabelButton>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Ghost Square (Drawer Toggle Pattern)"
                    code=r##"<LabelButton for_id="drawer-id" variant=Variant::Ghost square=true>
    // hamburger icon
</LabelButton>"##
                >
                    <div class="flex gap-2">
                        <LabelButton for_id="label-btn-demo2" variant=Variant::Ghost square=true>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-6 h-6 stroke-current">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </LabelButton>
                        <LabelButton for_id="label-btn-demo2" variant=Variant::Ghost circle=true>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="w-6 h-6 stroke-current">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </LabelButton>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors & Sizes"
                    code=r##"<LabelButton for_id="id" color=Color::Secondary size=Size::Small>"Small"</LabelButton>"##
                >
                    <div class="flex flex-wrap gap-2">
                        <LabelButton for_id="label-btn-demo3" color=Color::Primary size=Size::ExtraSmall>"XS"</LabelButton>
                        <LabelButton for_id="label-btn-demo3" color=Color::Secondary size=Size::Small>"SM"</LabelButton>
                        <LabelButton for_id="label-btn-demo3" color=Color::Accent>"MD"</LabelButton>
                        <LabelButton for_id="label-btn-demo3" color=Color::Info size=Size::Large>"LG"</LabelButton>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
