//! Comprehensive tests for Button component

use leptos::prelude::*;
use leptos::*;
use leptos_daisy::components::*;

#[test]
fn test_all_button_variants() {
    // Test all button variants compile correctly
    let variants = [
        ButtonVariant::Default,
        ButtonVariant::Primary,
        ButtonVariant::Secondary,
        ButtonVariant::Accent,
        ButtonVariant::Info,
        ButtonVariant::Success,
        ButtonVariant::Warning,
        ButtonVariant::Error,
    ];

    for variant in variants {
        let button = || {
            view! {
                <Button variant=variant>
                    {format!("Button {}", variant as i32)}
                </Button>
            }
        };
        let _view = button();
    }
}

#[test]
fn test_all_button_sizes() {
    // Test all button sizes compile correctly
    let sizes = [
        ButtonSize::Default,
        ButtonSize::ExtraSmall,
        ButtonSize::Small,
        ButtonSize::Large,
        ButtonSize::ExtraLarge,
        ButtonSize::Wide,
        ButtonSize::Block,
    ];

    for size in sizes {
        let button = || {
            view! {
                <Button size=size>
                    {format!("Size {}", size as i32)}
                </Button>
            }
        };
        let _view = button();
    }
}

#[test]
fn test_all_button_modifiers() {
    // Test all button modifiers compile correctly
    let button = || {
        view! {
            <Button
                outline=true
                ghost=true
                dash=true
                soft=true
                active=true
                link=true
                glass=true
                square=true
                circle=true
                no_animation=true
            >
                "All Modifiers"
            </Button>
        }
    };
    let _view = button();
}

#[test]
fn test_button_with_click_handler() {
    // Test button with click handler
    let clicked = RwSignal::new(false);
    let on_click = Callback::new(move |_| {
        clicked.set(true);
    });

    let button = || {
        view! {
            <Button on_click=on_click>
                "Click Me"
            </Button>
        }
    };

    let _view = button();
}

#[test]
fn test_button_combinations() {
    // Test various button combinations
    let combinations = [
        (ButtonVariant::Primary, ButtonSize::Large, true, false),
        (ButtonVariant::Success, ButtonSize::Small, false, true),
        (ButtonVariant::Warning, ButtonSize::ExtraLarge, true, true),
        (ButtonVariant::Error, ButtonSize::ExtraSmall, false, false),
    ];

    for (variant, size, outline, ghost) in combinations {
        let button = || {
            view! {
                <Button variant=variant size=size outline=outline ghost=ghost>
                    {format!("Btn {}-{}", variant as i32, size as i32)}
                </Button>
            }
        };
        let _view = button();
    }
}

#[test]
fn test_simple_button() {
    // Test simple button component
    let button = || {
        view! {
            <SimpleButton text="Simple Button".to_string() />
        }
    };
    let _view = button();
}

#[test]
fn test_button_with_custom_class() {
    // Test button with custom CSS class
    let button = || {
        view! {
            <Button class=Some("custom-class another-class".to_string())>
                "Custom Class Button"
            </Button>
        }
    };
    let _view = button();
}

#[test]
fn test_disabled_button() {
    // Test disabled button
    let button = || {
        view! {
            <Button disabled=true>
                "Disabled Button"
            </Button>
        }
    };
    let _view = button();
}

#[test]
fn test_button_with_children() {
    // Test button with complex children
    let button = || {
        view! {
            <Button variant=ButtonVariant::Primary>
                <span class="flex items-center gap-2">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"></path>
                    </svg>
                    "Button with Icon"
                </span>
            </Button>
        }
    };
    let _view = button();
}

#[test]
fn test_all_new_button_features() {
    // Test the newly added button features

    // Test ExtraLarge size
    let xl_button = || {
        view! {
            <Button size=ButtonSize::ExtraLarge>
                "XL Button"
            </Button>
        }
    };
    let _xl_view = xl_button();

    // Test dash style
    let dash_button = || {
        view! {
            <Button dash=true>
                "Dash Button"
            </Button>
        }
    };
    let _dash_view = dash_button();

    // Test soft style
    let soft_button = || {
        view! {
            <Button soft=true>
                "Soft Button"
            </Button>
        }
    };
    let _soft_view = soft_button();

    // Test square modifier
    let square_button = || {
        view! {
            <Button square=true>
                "Square"
            </Button>
        }
    };
    let _square_view = square_button();

    // Test circle modifier
    let circle_button = || {
        view! {
            <Button circle=true>
                "O"
            </Button>
        }
    };
    let _circle_view = circle_button();
}
