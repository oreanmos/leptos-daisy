//! Card component — daisyUI `card` + parts.
use crate::utils::class::class_signal;
use crate::variants::size::Size;
use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardVariant {
    #[default]
    Default,
    Bordered,
    Compact,
    Normal,
    Side,
}
impl CardVariant {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Bordered => "card-bordered",
            Self::Compact => "card-compact",
            Self::Normal => "card-normal",
            Self::Side => "card-side",
        }
    }
}

fn get_card_classes(variant: CardVariant, size: Option<Size>) -> Vec<&'static str> {
    let mut m = Vec::new();
    let vc = variant.cls();
    if !vc.is_empty() {
        m.push(vc);
    }
    if let Some(s) = size {
        match s {
            Size::ExtraSmall => m.push("card-xs"),
            Size::Small => m.push("card-sm"),
            Size::Medium => m.push("card-md"),
            Size::Large => m.push("card-lg"),
            Size::ExtraLarge => m.push("card-xl"),
        }
    }
    m
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional)] variant: CardVariant,
    #[prop(optional, into)] size: Option<Size>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let m = get_card_classes(variant, size);
    let cls = class_signal("card", &m, class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("card-header", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn CardBody(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("card-body", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("card-title", &[], class);
    view! { <h2 class=cls>{children()}</h2> }.add_any_attr(attrs)
}

#[component]
pub fn CardActions(
    children: Children,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<AnyAttribute>,
) -> impl IntoView {
    let cls = class_signal("card-actions", &[], class);
    view! { <div class=cls>{children()}</div> }.add_any_attr(attrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_variant_classes() {
        assert_eq!(CardVariant::Default.cls(), "");
        assert_eq!(CardVariant::Bordered.cls(), "card-bordered");
        assert_eq!(CardVariant::Compact.cls(), "card-compact");
        assert_eq!(CardVariant::Normal.cls(), "card-normal");
        assert_eq!(CardVariant::Side.cls(), "card-side");
    }

    #[test]
    fn test_card_classes_helper() {
        // Default variant, no size
        let m = get_card_classes(CardVariant::Default, None);
        assert!(m.is_empty());

        // Bordered variant, no size
        let m = get_card_classes(CardVariant::Bordered, None);
        assert_eq!(m, vec!["card-bordered"]);

        // Compact variant, no size
        let m = get_card_classes(CardVariant::Compact, None);
        assert_eq!(m, vec!["card-compact"]);

        // Default variant, small size
        let m = get_card_classes(CardVariant::Default, Some(Size::Small));
        assert_eq!(m, vec!["card-sm"]);

        // Bordered variant, large size
        let m = get_card_classes(CardVariant::Bordered, Some(Size::Large));
        assert_eq!(m, vec!["card-bordered", "card-lg"]);

        // Side variant, extra large size
        let m = get_card_classes(CardVariant::Side, Some(Size::ExtraLarge));
        assert_eq!(m, vec!["card-side", "card-xl"]);
    }
}
