//! Basic tests for DaisyUI components

use leptos::prelude::*;
use leptos::*;
use leptos_daisy::components::alert::{AlertDirection, AlertStyle, AlertVariant};
use leptos_daisy::components::*;

#[test]
fn test_button_component() {
    // Test that the Button component can be created with different variants
    let button = || {
        view! {
            <Button variant=ButtonVariant::Primary>
                "Test Button"
            </Button>
        }
    };

    // This should compile without errors
    let _view = button();
}

#[test]
fn test_alert_component() {
    // Test that the Alert component can be created with different variants
    let alert = || {
        view! {
            <Alert variant=AlertVariant::Success icon=true>
                "Test Alert"
            </Alert>
        }
    };

    // This should compile without errors
    let _view = alert();
}

#[test]
fn test_alert_styles() {
    // Test all alert styles compile
    let styles = [
        AlertStyle::Default,
        AlertStyle::Outline,
        AlertStyle::Dash,
        AlertStyle::Soft,
    ];

    for style in styles {
        let alert = || {
            view! {
                <Alert variant=AlertVariant::Info style=style>
                    {format!("Alert Style {}", style as i32)}
                </Alert>
            }
        };
        let _view = alert();
    }
}

#[test]
fn test_alert_directions() {
    // Test all alert directions compile
    let directions = [
        AlertDirection::Default,
        AlertDirection::Vertical,
        AlertDirection::Horizontal,
    ];

    for direction in directions {
        let alert = || {
            view! {
                <Alert variant=AlertVariant::Info direction=direction>
                    {format!("Alert Direction {}", direction as i32)}
                </Alert>
            }
        };
        let _view = alert();
    }
}

#[test]
fn test_card_component() {
    // Test that the Card component can be created with different variants
    let card = || {
        view! {
            <Card variant=CardVariant::Bordered>
                <CardBody>
                    <CardTitle>"Test Card"</CardTitle>
                    <p>"Test content"</p>
                </CardBody>
            </Card>
        }
    };

    // This should compile without errors
    let _view = card();
}

#[test]
fn test_button_variants() {
    // Test all button variants compile
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
fn test_alert_variants() {
    // Test all alert variants compile
    let variants = [
        AlertVariant::Info,
        AlertVariant::Success,
        AlertVariant::Warning,
        AlertVariant::Error,
    ];

    for variant in variants {
        let alert = || {
            view! {
                <Alert variant=variant>
                    {format!("Alert {}", variant as i32)}
                </Alert>
            }
        };
        let _view = alert();
    }
}

#[test]
fn test_card_variants() {
    // Test all card variants compile
    let variants = [
        CardVariant::Default,
        CardVariant::Bordered,
        CardVariant::Compact,
        CardVariant::Normal,
        CardVariant::Side,
    ];

    for variant in variants {
        let card = || {
            view! {
                <Card variant=variant>
                    {format!("Card {}", variant as i32)}
                </Card>
            }
        };
        let _view = card();
    }
}
