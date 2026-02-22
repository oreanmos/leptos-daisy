use leptos::prelude::*;
use leptos_daisyui::prelude::*;

#[test]
fn test_dock_item_accepts_strings() {
    // This test confirms that DockItem::new continues to accept strings,
    // which are now rendered safely as text instead of injected into inner_html.
    let xss_payload = "<img src=x onerror=alert(1)>";
    let item = DockItem::new("Label", xss_payload);

    assert_eq!(item.label, "Label");
}

#[test]
fn test_dock_item_accepts_view_factory() {
    // This test confirms that DockItem::new accepts view factories (closures).
    // This is required because View<T> might not be Clone (e.g. Svg).
    let item = DockItem::new("Label", || view! { <span>"Safe"</span> });

    assert_eq!(item.label, "Label");
}
