use crate::utils::class::class_signal;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridCols {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
impl GridCols {
    fn cls(&self) -> &'static str {
        match self {
            Self::One => "grid-cols-1",
            Self::Two => "grid-cols-2",
            Self::Three => "grid-cols-3",
            Self::Four => "grid-cols-4",
            Self::Five => "grid-cols-5",
            Self::Six => "grid-cols-6",
        }
    }
}

#[component]
pub fn Grid(
    children: Children,
    #[prop(optional)] cols: GridCols,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let cls = class_signal("grid", &[cols.cls(), "gap-4"], class);
    view! { <div class=cls>{children()}</div> }
}
