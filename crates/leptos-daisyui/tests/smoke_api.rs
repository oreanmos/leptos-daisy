use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[test]
fn prelude_core_symbols_are_available() {
    let _ = Color::Primary;
    let _ = Size::Large;
    let _ = Variant::Outline;
    let _ = State::Active;

    let merged = merge_classes(["btn", "btn-primary", "btn"]);
    assert_eq!(merged, "btn btn-primary");
}

#[test]
fn key_components_compile_in_view_macro() {
    let _view = view! {
        <div>
            <Button color=Color::Primary size=Size::Medium>
                "Primary"
            </Button>

            <Card class="bg-base-100" variant=CardVariant::Bordered>
                <CardBody>
                    <CardTitle>"Card title"</CardTitle>
                    <p>"Card body"</p>
                    <CardActions>
                        <Button variant=Variant::Outline color=Color::Secondary>
                            "Action"
                        </Button>
                    </CardActions>
                </CardBody>
            </Card>

            <Fab class="fab-bottom fab-end">
                <FabTrigger class="btn-primary">"+"</FabTrigger>
                <FabAction class="btn-secondary">"✉"</FabAction>
            </Fab>

            <ThemeController
                value="dark"
                class="toggle"
                aria_label="Enable dark theme"
            />
        </div>
    };
}
