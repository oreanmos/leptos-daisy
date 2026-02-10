use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <h1 class="text-3xl font-bold">"Tooltip"</h1>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Positions"</h2>
                <div class="flex flex-wrap gap-8 items-center justify-center py-8">
                    <Tooltip text="Top tooltip" position={TooltipPosition::Top}>
                        <Button>"Top"</Button>
                    </Tooltip>
                    <Tooltip text="Bottom tooltip" position={TooltipPosition::Bottom}>
                        <Button>"Bottom"</Button>
                    </Tooltip>
                    <Tooltip text="Left tooltip" position={TooltipPosition::Left}>
                        <Button>"Left"</Button>
                    </Tooltip>
                    <Tooltip text="Right tooltip" position={TooltipPosition::Right}>
                        <Button>"Right"</Button>
                    </Tooltip>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Colors"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <Tooltip text="Primary" color={Color::Primary}>
                        <Button color={Color::Primary}>"Primary"</Button>
                    </Tooltip>
                    <Tooltip text="Secondary" color={Color::Secondary}>
                        <Button color={Color::Secondary}>"Secondary"</Button>
                    </Tooltip>
                    <Tooltip text="Accent" color={Color::Accent}>
                        <Button color={Color::Accent}>"Accent"</Button>
                    </Tooltip>
                    <Tooltip text="Info" color={Color::Info}>
                        <Button color={Color::Info}>"Info"</Button>
                    </Tooltip>
                    <Tooltip text="Success" color={Color::Success}>
                        <Button color={Color::Success}>"Success"</Button>
                    </Tooltip>
                    <Tooltip text="Warning" color={Color::Warning}>
                        <Button color={Color::Warning}>"Warning"</Button>
                    </Tooltip>
                    <Tooltip text="Error" color={Color::Error}>
                        <Button color={Color::Error}>"Error"</Button>
                    </Tooltip>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Open State"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <Tooltip text="Always visible" open=true>
                        <Button variant={Variant::Outline}>"Open"</Button>
                    </Tooltip>
                    <Tooltip text="Always visible top" position={TooltipPosition::Top} open=true color={Color::Primary}>
                        <Button color={Color::Primary}>"Open Top"</Button>
                    </Tooltip>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"On Different Elements"</h2>
                <div class="flex flex-wrap gap-4 items-center">
                    <Tooltip text="Click to save">
                        <Button color={Color::Primary}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
                            </svg>
                            "Save"
                        </Button>
                    </Tooltip>
                    <Tooltip text="User profile">
                        <Avatar>
                            <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=User" alt="User" />
                        </Avatar>
                    </Tooltip>
                    <Tooltip text="Notifications">
                        <Button variant={Variant::Ghost} class="btn-circle">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                            </svg>
                        </Button>
                    </Tooltip>
                    <Tooltip text="Copy to clipboard">
                        <Kbd>"Ctrl + C"</Kbd>
                    </Tooltip>
                </div>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Long Tooltip Text"</h2>
                <Tooltip text="This is a longer tooltip text that provides more detailed information about the element when hovered.">
                    <Button variant={Variant::Outline}>"Hover for details"</Button>
                </Tooltip>
            </section>

            <section>
                <h2 class="text-xl font-semibold mb-4">"Form Elements"</h2>
                <div class="space-y-4 max-w-sm">
                    <div>
                        <Tooltip text="Enter your full name as it appears on your ID" position={TooltipPosition::Right}>
                            <Label>"Full Name"</Label>
                        </Tooltip>
                        <Input placeholder="John Doe" />
                    </div>
                    <div>
                        <Tooltip text="We'll never share your email" position={TooltipPosition::Right}>
                            <Label>"Email"</Label>
                        </Tooltip>
                        <Input placeholder="john@example.com" />
                    </div>
                </div>
            </section>
        </div>
    }
}
