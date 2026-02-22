use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContainerSize {
    #[default]
    Default,
    Small,
    Medium,
    Large,
    ExtraLarge,
}
impl ContainerSize {
    fn cls(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Small => "max-w-sm",
            Self::Medium => "max-w-md",
            Self::Large => "max-w-lg",
            Self::ExtraLarge => "max-w-xl",
        }
    }
}

#[component]
pub fn Container(
    children: Children,
    #[prop(optional)] size: ContainerSize,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let mut m: Vec<&str> = Vec::new();
    let sc = size.cls();
    if !sc.is_empty() {
        m.push(sc);
    }
    let cls = class_signal("container mx-auto", &m, class);
    view! { <div class=cls>{children()}</div> }
}
