use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    let (framework, set_framework) = signal("leptos".to_string());

    view! {
        <div class="space-y-10">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Select"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Dropdown controls with complete style variants and semantic form attributes."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                    code=r##"<Select>
    <SelectOption disabled=true selected=true>"Pick a framework"</SelectOption>
    <SelectOption value="leptos">"Leptos"</SelectOption>
    <SelectOption value="yew">"Yew"</SelectOption>
</Select>

<Select color={Color::Primary}>
    <SelectOption selected=true>"Primary select"</SelectOption>
    <SelectOption>"Option A"</SelectOption>
</Select>

<Select color={Color::Secondary}><SelectOption>"Secondary"</SelectOption></Select>
<Select color={Color::Accent}><SelectOption>"Accent"</SelectOption></Select>
<Select color={Color::Info}><SelectOption>"Info"</SelectOption></Select>
<Select color={Color::Success}><SelectOption>"Success"</SelectOption></Select>
<Select color={Color::Warning}><SelectOption>"Warning"</SelectOption></Select>
<Select color={Color::Error}><SelectOption>"Error"</SelectOption></Select>"##
                >
                    <div class="flex flex-col gap-3 max-w-md">
                        <Select>
                            <SelectOption disabled=true selected=true>"Pick a framework"</SelectOption>
                            <SelectOption value="leptos">"Leptos"</SelectOption>
                            <SelectOption value="yew">"Yew"</SelectOption>
                            <SelectOption value="dioxus">"Dioxus"</SelectOption>
                        </Select>
                        <Select color={Color::Primary}>
                            <SelectOption selected=true>"Primary select"</SelectOption>
                            <SelectOption>"Option A"</SelectOption>
                            <SelectOption>"Option B"</SelectOption>
                        </Select>
                        <Select color={Color::Secondary}><SelectOption>"Secondary"</SelectOption></Select>
                        <Select color={Color::Accent}><SelectOption>"Accent"</SelectOption></Select>
                        <Select color={Color::Info}><SelectOption>"Info"</SelectOption></Select>
                        <Select color={Color::Success}><SelectOption>"Success"</SelectOption></Select>
                        <Select color={Color::Warning}><SelectOption>"Warning"</SelectOption></Select>
                        <Select color={Color::Error}><SelectOption>"Error"</SelectOption></Select>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Sizes"
                    code=r##"<Select size={Size::ExtraSmall}>
    <SelectOption selected=true>"Extra Small"</SelectOption>
</Select>
<Select size={Size::Small}>
    <SelectOption selected=true>"Small"</SelectOption>
</Select>
<Select size={Size::Medium}>
    <SelectOption selected=true>"Medium"</SelectOption>
</Select>
<Select size={Size::Large}>
    <SelectOption selected=true>"Large"</SelectOption>
</Select>
<Select size={Size::ExtraLarge}>
    <SelectOption selected=true>"Extra Large"</SelectOption>
</Select>"##
                >
                    <div class="flex flex-col gap-3 max-w-md">
                        <Select size={Size::ExtraSmall}>
                            <SelectOption selected=true>"Extra Small"</SelectOption>
                        </Select>
                        <Select size={Size::Small}>
                            <SelectOption selected=true>"Small"</SelectOption>
                        </Select>
                        <Select size={Size::Medium}>
                            <SelectOption selected=true>"Medium"</SelectOption>
                        </Select>
                        <Select size={Size::Large}>
                            <SelectOption selected=true>"Large"</SelectOption>
                        </Select>
                        <Select size={Size::ExtraLarge}>
                            <SelectOption selected=true>"Extra Large"</SelectOption>
                        </Select>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Styles and states"
                    code=r##"<Select variant={Variant::Outline}>
    <SelectOption selected=true>"Bordered / outline style"</SelectOption>
</Select>
<Select variant={Variant::Ghost}>
    <SelectOption selected=true>"Ghost style"</SelectOption>
</Select>
<Select disabled=true>
    <SelectOption selected=true>"Disabled select"</SelectOption>
</Select>"##
                >
                    <div class="flex flex-col gap-3 max-w-md">
                        <Select variant={Variant::Outline}>
                            <SelectOption selected=true>"Bordered / outline style"</SelectOption>
                        </Select>
                        <Select variant={Variant::Ghost}>
                            <SelectOption selected=true>"Ghost style"</SelectOption>
                        </Select>
                        <Select disabled=true>
                            <SelectOption selected=true>"Disabled select"</SelectOption>
                        </Select>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Reactive selection"
                    code=r##"let (framework, set_framework) = signal("leptos".to_string());

view! {
    <Select
        name="framework"
        value=framework
        on:change=move |ev| set_framework.set(event_target_value(&ev))
    >
        <SelectOption value="leptos">"Leptos"</SelectOption>
        <SelectOption value="yew">"Yew"</SelectOption>
        <SelectOption value="dioxus">"Dioxus"</SelectOption>
        <SelectOption value="sycamore">"Sycamore"</SelectOption>
    </Select>
    <p class="text-sm text-base-content/70">
        {move || format!("Selected: {}", framework.get())}
    </p>
}"##
                >
                    <div class="max-w-md space-y-3">
                        <Select
                            name="framework"
                            value=framework
                            on:change=move |ev| set_framework.set(event_target_value(&ev))
                        >
                            <SelectOption value="leptos">"Leptos"</SelectOption>
                            <SelectOption value="yew">"Yew"</SelectOption>
                            <SelectOption value="dioxus">"Dioxus"</SelectOption>
                            <SelectOption value="sycamore">"Sycamore"</SelectOption>
                        </Select>
                        <p class="text-sm text-base-content/70">
                            {move || format!("Selected: {}", framework.get())}
                        </p>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
