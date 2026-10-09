//! Regression: reactive prop reads must not panic once the signal that backs
//! a prop has been disposed (e.g. an island torn down by an aborted stream).
//! A disposed read yields the default (`None` / `false`).
#![cfg(feature = "ssr")]

use leptos::prelude::*;
use leptos_daisyui::prelude::*;

/// A `MaybeProp` whose owner (and so its signal) is already disposed.
fn gone<T: Clone + Send + Sync + 'static>(v: T) -> MaybeProp<T> {
    let owner = Owner::new();
    let p = owner.with(|| MaybeProp::derive(move || Some(v.clone())));
    owner.cleanup();
    p
}

/// Renders the view built by `f` to HTML; panics on a disposed read.
fn render<V: leptos::tachys::view::RenderHtml>(f: impl FnOnce() -> V) -> String {
    let owner = Owner::new();
    owner.set();
    let html = f().to_html();
    owner.cleanup();
    html
}

#[test]
fn button_survives_disposed_props() {
    let html = render(|| {
        view! {
            <Button
                id=gone("i".to_string())
                name=gone("n".to_string())
                value=gone("v".to_string())
                aria_label=gone("a".to_string())
                disabled=gone(true)
                button_type=gone("submit".to_string())
                class=gone("x".to_string())
            >
                "ok"
            </Button>
        }
    });
    assert!(html.contains("<button"));
    assert!(!html.contains("disabled"));
}

#[test]
fn form_controls_survive_disposed_props() {
    let html = render(|| {
        view! {
            <Input
                id=gone("i".to_string())
                name=gone("n".to_string())
                value=gone("v".to_string())
                placeholder=gone("p".to_string())
                aria_label=gone("a".to_string())
                disabled=gone(true)
                required=gone(true)
                readonly=gone(true)
                class=gone("x".to_string())
            />
            <Textarea
                id=gone("i".to_string())
                value=gone("v".to_string())
                disabled=gone(true)
                required=gone(true)
                class=gone("x".to_string())
            />
            <Select id=gone("i".to_string()) disabled=gone(true) class=gone("x".to_string())>
                <SelectOption value=gone("v".to_string()) selected=gone(true)>"o"</SelectOption>
            </Select>
            <Toggle checked=gone(true) disabled=gone(true) class=gone("x".to_string()) />
            <Checkbox checked=gone(true) disabled=gone(true) class=gone("x".to_string()) />
        }
    });
    assert!(html.contains("<input"));
    assert!(!html.contains("disabled"));
    assert!(!html.contains("checked"));
}

#[test]
fn overlays_and_navigation_survive_disposed_props() {
    let html = render(|| {
        view! {
            <Modal open=gone(true) id=gone("m".to_string()) class=gone("x".to_string())>
                <ModalBox>"body"</ModalBox>
            </Modal>
            <Tabs>
                <Tab active=gone(true) class=gone("x".to_string())>"t"</Tab>
            </Tabs>
            <Dropdown>
                <DropdownContent>
                    <DropdownItem active=gone(true) disabled=gone(true) href=gone("/x".to_string())>
                        "i"
                    </DropdownItem>
                </DropdownContent>
            </Dropdown>
        }
    });
    assert!(html.contains("<dialog"));
    assert!(!html.contains("tab-active"));
}
