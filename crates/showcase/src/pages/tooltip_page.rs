use crate::components::component_preview::ComponentPreview;
use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <header class="space-y-3">
                <h1 class="text-3xl font-bold">"Tooltip"</h1>
                <p class="text-base-content/70 max-w-3xl">
                    "Tooltip displays a data-tip message when hovering over the children elements."
                </p>
            </header>

            <section class="space-y-4">
                <ComponentPreview
                    title="Positions"
                    code=r#"<Tooltip tip="Top tooltip" position={TooltipPosition::Top}>
    <Button>"Top"</Button>
</Tooltip>

<Tooltip tip="Bottom tooltip" position={TooltipPosition::Bottom}>
    <Button>"Bottom"</Button>
</Tooltip>

<Tooltip tip="Left tooltip" position={TooltipPosition::Left}>
    <Button>"Left"</Button>
</Tooltip>

<Tooltip tip="Right tooltip" position={TooltipPosition::Right}>
    <Button>"Right"</Button>
</Tooltip>"#
                >
                    <div class="flex flex-wrap gap-8 items-center justify-center py-8">
                        <Tooltip tip="Top tooltip" position={TooltipPosition::Top}>
                            <Button>"Top"</Button>
                        </Tooltip>
                        <Tooltip tip="Bottom tooltip" position={TooltipPosition::Bottom}>
                            <Button>"Bottom"</Button>
                        </Tooltip>
                        <Tooltip tip="Left tooltip" position={TooltipPosition::Left}>
                            <Button>"Left"</Button>
                        </Tooltip>
                        <Tooltip tip="Right tooltip" position={TooltipPosition::Right}>
                            <Button>"Right"</Button>
                        </Tooltip>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Colors"
                     code=r#"<Tooltip tip="Primary" color={Color::Primary}>
    <Button color={Color::Primary}>"Primary"</Button>
</Tooltip>
<Tooltip tip="Secondary" color={Color::Secondary}>
    <Button color={Color::Secondary}>"Secondary"</Button>
</Tooltip>
<Tooltip tip="Accent" color={Color::Accent}>
    <Button color={Color::Accent}>"Accent"</Button>
</Tooltip>
<Tooltip tip="Info" color={Color::Info}>
    <Button color={Color::Info}>"Info"</Button>
</Tooltip>
<Tooltip tip="Success" color={Color::Success}>
    <Button color={Color::Success}>"Success"</Button>
</Tooltip>
<Tooltip tip="Warning" color={Color::Warning}>
    <Button color={Color::Warning}>"Warning"</Button>
</Tooltip>
<Tooltip tip="Error" color={Color::Error}>
    <Button color={Color::Error}>"Error"</Button>
</Tooltip>"#
                >
                    <div class="flex flex-wrap gap-4 items-center justify-center">
                        <Tooltip tip="Primary" color={Color::Primary}>
                            <Button color={Color::Primary}>"Primary"</Button>
                        </Tooltip>
                        <Tooltip tip="Secondary" color={Color::Secondary}>
                            <Button color={Color::Secondary}>"Secondary"</Button>
                        </Tooltip>
                        <Tooltip tip="Accent" color={Color::Accent}>
                            <Button color={Color::Accent}>"Accent"</Button>
                        </Tooltip>
                        <Tooltip tip="Info" color={Color::Info}>
                            <Button color={Color::Info}>"Info"</Button>
                        </Tooltip>
                        <Tooltip tip="Success" color={Color::Success}>
                            <Button color={Color::Success}>"Success"</Button>
                        </Tooltip>
                        <Tooltip tip="Warning" color={Color::Warning}>
                            <Button color={Color::Warning}>"Warning"</Button>
                        </Tooltip>
                        <Tooltip tip="Error" color={Color::Error}>
                            <Button color={Color::Error}>"Error"</Button>
                        </Tooltip>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="Open State"
                    code=r#"<Tooltip tip="Always visible" open=true>
    <Button variant={Variant::Outline}>"Open"</Button>
</Tooltip>

<Tooltip tip="Always visible top" position={TooltipPosition::Top} open=true color={Color::Primary}>
    <Button color={Color::Primary}>"Open Top"</Button>
</Tooltip>"#
                    description="Force the tooltip to be open with the `open` prop."
                >
                    <div class="flex flex-wrap gap-4 items-center justify-center pt-8">
                        <Tooltip tip="Always visible" open=true>
                            <Button variant={Variant::Outline}>"Open"</Button>
                        </Tooltip>
                        <Tooltip tip="Always visible top" position={TooltipPosition::Top} open=true color={Color::Primary}>
                            <Button color={Color::Primary}>"Open Top"</Button>
                        </Tooltip>
                    </div>
                </ComponentPreview>
            </section>

            <section class="space-y-4">
                <ComponentPreview
                    title="On Different Elements"
                     code=r#"<Tooltip tip="Click to save">
    <Button color={Color::Primary}>
        // Icon SVG
        "Save"
    </Button>
</Tooltip>

<Tooltip tip="User profile">
    <div class="avatar placeholder">
        <div class="bg-neutral text-neutral-content rounded-full w-12">
            <span>"UI"</span>
        </div>
    </div>
</Tooltip>

<Tooltip tip="Notifications">
    <Button variant={Variant::Ghost} class="btn-circle">
        // Icon SVG
    </Button>
</Tooltip>

<Tooltip tip="Copy to clipboard">
    <kbd class="kbd">"Ctrl + C"</kbd>
</Tooltip>"#
                >
                    <div class="flex flex-wrap gap-8 items-center justify-center">
                        <Tooltip tip="Click to save">
                            <Button color={Color::Primary}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
                                </svg>
                                "Save"
                            </Button>
                        </Tooltip>
                        <Tooltip tip="User profile">
                            <div class="avatar placeholder">
                                <div class="bg-neutral text-neutral-content rounded-full w-12">
                                    <span>"UI"</span>
                                </div>
                            </div>
                        </Tooltip>
                        <Tooltip tip="Notifications">
                            <Button variant={Variant::Ghost} class="btn-circle">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                                </svg>
                            </Button>
                        </Tooltip>
                        <Tooltip tip="Copy to clipboard">
                            <kbd class="kbd">"Ctrl + C"</kbd>
                        </Tooltip>
                    </div>
                </ComponentPreview>
            </section>
        </div>
    }
}
