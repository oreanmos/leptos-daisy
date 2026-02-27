//! Sort controller — reactive sort state for tables.
//!
//! Provides `SortController` + `use_sort()` hook for managing column sort
//! state, plus `SortableHeaderCell` for clickable sortable headers.

use leptos::prelude::*;

/// Reactive controller for managing table sort state.
///
/// # Example
///
/// ```rust,ignore
/// let sort = use_sort();
///
/// view! {
///     <Table>
///         <TableHead>
///             <TableRow>
///                 <SortableHeaderCell controller=sort field="name">"Name"</SortableHeaderCell>
///                 <SortableHeaderCell controller=sort field="email">"Email"</SortableHeaderCell>
///             </TableRow>
///         </TableHead>
///     </Table>
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SortController {
    column: RwSignal<String>,
    ascending: RwSignal<bool>,
}

impl SortController {
    /// Create a new sort controller with no active sort.
    pub fn new() -> Self {
        Self {
            column: RwSignal::new(String::new()),
            ascending: RwSignal::new(true),
        }
    }

    /// Toggle sort on a column. If the column is already active, flips direction.
    /// If a different column, sets it as active (ascending).
    pub fn toggle(&self, col: &str) {
        if self.column.get_untracked() == col {
            self.ascending.update(|v| *v = !*v);
        } else {
            self.column.set(col.to_string());
            self.ascending.set(true);
        }
    }

    /// Returns a sort direction indicator for the given column.
    ///
    /// Returns `" \u{25B2}"` (up) if ascending, `" \u{25BC}"` (down) if descending,
    /// or `""` if the column is not active.
    pub fn indicator(&self, col: &str) -> String {
        if self.column.get() == col {
            if self.ascending.get() {
                " \u{25B2}".to_string()
            } else {
                " \u{25BC}".to_string()
            }
        } else {
            String::new()
        }
    }

    /// Whether the given column is the currently sorted column.
    pub fn is_active(&self, col: &str) -> bool {
        self.column.get() == col
    }

    /// Returns the current sort column name (empty if none).
    pub fn column(&self) -> String {
        self.column.get()
    }

    /// Whether the current sort direction is ascending.
    pub fn is_ascending(&self) -> bool {
        self.ascending.get()
    }

    /// Returns a sort parameter string suitable for API query params.
    ///
    /// Returns `Some("+column")` for ascending, `Some("-column")` for descending,
    /// or `None` if no column is active.
    pub fn sort_param(&self) -> Option<String> {
        let col = self.column.get();
        if col.is_empty() {
            None
        } else if self.ascending.get() {
            Some(format!("+{col}"))
        } else {
            Some(format!("-{col}"))
        }
    }

    /// Get the column signal for reactive access.
    pub fn column_signal(&self) -> RwSignal<String> {
        self.column
    }

    /// Get the ascending signal for reactive access.
    pub fn ascending_signal(&self) -> RwSignal<bool> {
        self.ascending
    }
}

impl Default for SortController {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new [`SortController`].
pub fn use_sort() -> SortController {
    SortController::new()
}

/// A clickable table header cell with sort direction indicator.
///
/// Renders a `<th>` that toggles sort on click and displays an arrow
/// indicator for the active sort column.
#[component]
pub fn SortableHeaderCell(
    controller: SortController,
    /// The field/column ID to sort by when clicked.
    #[prop(into)]
    field: String,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let field_click = field.clone();
    let field_indicator = field.clone();

    let on_click = move |_| {
        controller.toggle(&field_click);
    };

    let indicator = move || controller.indicator(&field_indicator);

    let cls = move || {
        let base = "cursor-pointer select-none hover:bg-base-200";
        match class.get() {
            Some(extra) => format!("{base} {extra}"),
            None => base.to_string(),
        }
    };

    view! {
        <th class=cls on:click=on_click>
            {children()}
            <span class="text-xs">{indicator}</span>
        </th>
    }
}
