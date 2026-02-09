//! Button Component Demo
//!
//! This example demonstrates ALL button variants, sizes, and modifiers.

use leptos::prelude::*;
use leptos::*;
use leptos_daisy::components::*;

/// Comprehensive button demo showing all features
#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <div class="p-4 space-y-6">
            <h1 class="text-3xl font-bold text-center">"🎯 Complete Button Component Demo"</h1>

            {/* Variants Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"🎨 All Variants (8 total)"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button variant=ButtonVariant::Default>"Default"</Button>
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Accent>"Accent"</Button>
                    <Button variant=ButtonVariant::Info>"Info"</Button>
                    <Button variant=ButtonVariant::Success>"Success"</Button>
                    <Button variant=ButtonVariant::Warning>"Warning"</Button>
                    <Button variant=ButtonVariant::Error>"Error"</Button>
                </div>
            </div>

            {/* Sizes Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"📏 All Sizes (7 total)"</h2>
                <div class="flex flex-wrap gap-2 items-end">
                    <Button size=ButtonSize::ExtraSmall>"XS"</Button>
                    <Button size=ButtonSize::Small>"Small"</Button>
                    <Button>"Default"</Button>
                    <Button size=ButtonSize::Large>"Large"</Button>
                    <Button size=ButtonSize::ExtraLarge>"XL"</Button>
                    <Button size=ButtonSize::Wide>"Wide Button"</Button>
                    <Button size=ButtonSize::Block>"Block Button"</Button>
                </div>
            </div>

            {/* Modifiers Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"⚙️ All Modifiers (11 total)"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button outline=true>"Outline"</Button>
                    <Button ghost=true>"Ghost"</Button>
                    <Button dash=true>"Dash"</Button>
                    <Button soft=true>"Soft"</Button>
                    <Button active=true>"Active"</Button>
                    <Button disabled=true>"Disabled"</Button>
                    <Button link=true>"Link"</Button>
                    <Button glass=true>"Glass"</Button>
                    <Button square=true>"S"</Button>
                    <Button circle=true>"O"</Button>
                    <Button no_animation=true>"No Anim"</Button>
                </div>
            </div>

            {/* Combinations Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"🔧 Powerful Combinations"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Large outline=true>
                        "Primary XL Outline"
                    </Button>
                    <Button variant=ButtonVariant::Success size=ButtonSize::Small ghost=true>
                        "Success SM Ghost"
                    </Button>
                    <Button variant=ButtonVariant::Warning dash=true square=true>
                        "W"
                    </Button>
                    <Button variant=ButtonVariant::Error soft=true circle=true>
                        "E"
                    </Button>
                    <Button variant=ButtonVariant::Info size=ButtonSize::ExtraLarge glass=true>
                        "Info XL Glass"
                    </Button>
                </div>
            </div>

            {/* Interactive Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"🖱️ Interactive Buttons"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button
                        variant=ButtonVariant::Primary
                        on_click=|_| log::info!("Primary button clicked!")
                    >
                        "Click Me (Primary)"
                    </Button>
                    <Button
                        variant=ButtonVariant::Success
                        size=ButtonSize::Large
                        on_click=|_| log::info!("Success button clicked!")
                    >
                        "Click Me (Success)"
                    </Button>
                    <Button
                        variant=ButtonVariant::Warning
                        outline=true
                        on_click=|_| log::info!("Warning button clicked!")
                    >
                        "Click Me (Warning)"
                    </Button>
                </div>
            </div>

            {/* Custom Classes Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"🎨 Custom Styling"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button class=Some("btn btn-primary custom-class".to_string())>
                        "Custom Class"
                    </Button>
                    <Button class=Some("btn btn-secondary extra-padding".to_string())>
                        "Extra Padding"
                    </Button>
                </div>
            </div>

            {/* Simple Button Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"📝 Simple Button Component"</h2>
                <div class="flex flex-wrap gap-2">
                    <SimpleButton text="Simple Primary".to_string() variant=ButtonVariant::Primary />
                    <SimpleButton text="Simple Success".to_string() variant=ButtonVariant::Success />
                    <SimpleButton
                        text="Simple Warning".to_string()
                        variant=ButtonVariant::Warning
                        size=ButtonSize::Small
                    />
                </div>
            </div>

            {/* Complex Children Section */}
            <div class="space-y-2">
                <h2 class="text-xl font-semibold">"🔥 Complex Children"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button variant=ButtonVariant::Primary>
                        <span class="flex items-center gap-2">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"></path>
                            </svg>
                            "Button with Icon"
                        </span>
                    </Button>
                    <Button variant=ButtonVariant::Info>
                        <div class="flex flex-col items-center">
                            <span>"Multi-line"</span>
                            <span class="text-xs">"Button Content"</span>
                        </div>
                    </Button>
                </div>
            </div>

            {/* Statistics Section */}
            <div class="mt-6 p-4 bg-gray-100 rounded-lg">
                <h3 class="text-lg font-semibold mb-2">"📊 Button Component Statistics"</h3>
                <ul class="space-y-1 text-sm">
                    <li>"✅ 8 Button Variants Implemented"</li>
                    <li>"✅ 7 Button Sizes Supported"</li>
                    <li>"✅ 11 Button Modifiers Available"</li>
                    <li>"✅ Click Event Handling"</li>
                    <li>"✅ Custom CSS Classes"</li>
                    <li>"✅ Flexible Children Support"</li>
                    <li>"✅ Type-Safe API"</li>
                    <li>"✅ Comprehensive Testing"</li>
                    <li>"✅ DaisyUI Compatibility"</li>
                    <li>"✅ Leptos Integration"</li>
                </ul>
            </div>
        </div>
    }
}
