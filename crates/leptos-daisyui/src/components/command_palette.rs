//! CommandPalette component — a Cmd+K / Ctrl+K search palette.
//!
//! Provides a modal search interface with keyboard navigation, item grouping,
//! and optional global keyboard shortcut.

use leptos::ev;
use leptos::prelude::*;

/// A single item in the command palette.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandPaletteItem {
    pub id: String,
    pub label: String,
    pub group: Option<String>,
    pub description: Option<String>,
}

/// A Cmd+K / Ctrl+K command palette with search, grouping, and keyboard navigation.
///
/// # Example
///
/// ```rust,ignore
/// let open = RwSignal::new(false);
/// let items = vec![
///     CommandPaletteItem { id: "home".into(), label: "Go Home".into(), group: Some("Navigation".into()), description: None },
///     CommandPaletteItem { id: "settings".into(), label: "Settings".into(), group: Some("Navigation".into()), description: Some("App settings".into()) },
/// ];
///
/// view! {
///     <CommandPalette
///         open=open
///         items=items
///         on_select=Callback::new(move |id: String| { /* handle selection */ })
///     />
/// }
/// ```
#[component]
pub fn CommandPalette(
    open: RwSignal<bool>,
    #[prop(into)] items: Signal<Vec<CommandPaletteItem>>,
    on_select: Callback<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, default = true)] keyboard_shortcut: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);

    // Filtered items based on search query
    let filtered = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        let all_items = items.get();
        if q.is_empty() {
            all_items
        } else {
            all_items
                .into_iter()
                .filter(|item| {
                    item.label.to_lowercase().contains(&q)
                        || item
                            .description
                            .as_ref()
                            .is_some_and(|d| d.to_lowercase().contains(&q))
                })
                .collect()
        }
    });

    // Reset selection when query changes
    Effect::new(move || {
        let _ = query.get();
        set_selected_index.set(0);
    });

    // Reset query when palette closes
    Effect::new(move || {
        if !open.get() {
            set_query.set(String::new());
            set_selected_index.set(0);
        }
    });

    // Global keyboard shortcut (Cmd+K / Ctrl+K)
    let _ = keyboard_shortcut; // used only in csr/hydrate
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    if keyboard_shortcut {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;

        let cb =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
                if (ev.meta_key() || ev.ctrl_key()) && ev.key() == "k" {
                    ev.prevent_default();
                    open.update(|v| *v = !*v);
                }
            });

        if let Some(win) = leptos::prelude::window().dyn_ref::<web_sys::EventTarget>() {
            let _ = win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
        }
        cb.forget();
    }

    // Keyboard navigation within the palette
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        let items = filtered.get();
        let len = items.len();
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                set_selected_index.update(|i| *i = if len > 0 { (*i + 1) % len } else { 0 });
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|i| {
                    *i = if len > 0 {
                        i.checked_sub(1).unwrap_or(len - 1)
                    } else {
                        0
                    };
                });
            }
            "Enter" => {
                ev.prevent_default();
                let idx = selected_index.get_untracked();
                if let Some(item) = items.get(idx) {
                    on_select.run(item.id.clone());
                    open.set(false);
                }
            }
            "Escape" => {
                ev.prevent_default();
                open.set(false);
            }
            _ => {}
        }
    };

    let handle_input = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        set_query.set(value);
    };

    let placeholder_text = placeholder
        .get_untracked()
        .unwrap_or_else(|| "Search...".to_string());

    let extra_class = class;

    view! {
        <Show when=move || open.get()>
            <div class=move || extra_class.get().unwrap_or_default()>
                // Backdrop
                <div
                    class="fixed inset-0 bg-black/50 z-50"
                    on:click=move |_| open.set(false)
                ></div>
                // Palette
                <div class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]">
                    <div class="bg-base-100 rounded-box shadow-2xl w-full max-w-lg mx-4 overflow-hidden border border-base-300">
                        // Search input
                        <div class="flex items-center gap-2 px-4 py-3 border-b border-base-300">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 opacity-50">
                                <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
                            </svg>
                            <input
                                class="input input-ghost w-full focus:outline-none"
                                placeholder=placeholder_text.clone()
                                autofocus=true
                                prop:value=move || query.get()
                                on:input=handle_input
                                on:keydown=handle_keydown
                            />
                            <kbd class="kbd kbd-sm">"Esc"</kbd>
                        </div>
                        // Results
                        <div class="max-h-80 overflow-y-auto p-2">
                            {move || {
                                let items = filtered.get();
                                if items.is_empty() {
                                    view! {
                                        <div class="text-center py-8 text-base-content/50">
                                            "No results found"
                                        </div>
                                    }.into_any()
                                } else {
                                    let sel = selected_index.get();
                                    let mut current_group: Option<String> = None;
                                    let views: Vec<_> = items.iter().enumerate().map(|(i, item)| {
                                        let id = item.id.clone();
                                        let is_selected = i == sel;

                                        let group_header = if item.group != current_group {
                                            current_group.clone_from(&item.group);
                                            item.group.as_ref().map(|g| view! {
                                                <div class="text-xs font-semibold text-base-content/50 px-3 pt-3 pb-1">
                                                    {g.clone()}
                                                </div>
                                            })
                                        } else {
                                            None
                                        };

                                        let desc = item.description.clone();

                                        view! {
                                            {group_header}
                                            <button
                                                class="w-full text-left px-3 py-2 rounded-btn flex flex-col gap-0.5 cursor-pointer"
                                                class:bg-base-200=is_selected
                                                on:click=move |_| {
                                                    on_select.run(id.clone());
                                                    open.set(false);
                                                }
                                                on:mouseenter={
                                                    let set_idx = set_selected_index;
                                                    move |_| set_idx.set(i)
                                                }
                                            >
                                                <span class="font-medium">{item.label.clone()}</span>
                                                {desc.map(|d| view! {
                                                    <span class="text-xs text-base-content/60">{d}</span>
                                                })}
                                            </button>
                                        }
                                    }).collect();
                                    views.collect_view().into_any()
                                }
                            }}
                        </div>
                        // Footer hint
                        <div class="flex items-center gap-4 px-4 py-2 border-t border-base-300 text-xs text-base-content/50">
                            <span class="flex items-center gap-1">
                                <kbd class="kbd kbd-xs">{"\u{2191}"}</kbd>
                                <kbd class="kbd kbd-xs">{"\u{2193}"}</kbd>
                                " navigate"
                            </span>
                            <span class="flex items-center gap-1">
                                <kbd class="kbd kbd-xs">{"\u{21B5}"}</kbd>
                                " select"
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
