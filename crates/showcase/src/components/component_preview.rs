use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::CodeBlock;

static TAB_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn ComponentPreview(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(into)] code: String,
    children: Children,
) -> impl IntoView {
    let tab_id = TAB_COUNTER.fetch_add(1, Ordering::Relaxed);
    let viewport = RwSignal::new("100%".to_string());

    view! {
        <div class="flex flex-col gap-4 w-full">
            {move || title.clone().map(|t| view! { <h2 class="text-2xl font-bold">{t}</h2> })}
            {move || description.clone().map(|d| view! { <p class="text-base-content/70">{d}</p> })}

            <div role="tablist" class="tabs tabs-lift">
                <input type="radio" name={format!("tabs_{}", tab_id)} role="tab" class="tab" aria-label="Preview" checked=true />
                <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
                    <div class="flex justify-end gap-1 mb-3">
                        <button
                            class=move || format!("btn btn-xs {}", if viewport.get() == "375px" { "btn-active" } else { "btn-ghost" })
                            on:click=move |_| viewport.set("375px".to_string())
                        >
                            "Mobile"
                        </button>
                        <button
                            class=move || format!("btn btn-xs {}", if viewport.get() == "768px" { "btn-active" } else { "btn-ghost" })
                            on:click=move |_| viewport.set("768px".to_string())
                        >
                            "Tablet"
                        </button>
                        <button
                            class=move || format!("btn btn-xs {}", if viewport.get() == "100%" { "btn-active" } else { "btn-ghost" })
                            on:click=move |_| viewport.set("100%".to_string())
                        >
                            "Desktop"
                        </button>
                    </div>
                    <div
                        class="preview border border-base-200 bg-base-100 rounded-box min-h-[10rem] flex flex-col items-center justify-center p-4 mx-auto transition-all duration-300"
                        style=move || format!("max-width: {}", viewport.get())
                    >
                        {children()}
                    </div>
                </div>

                <input type="radio" name={format!("tabs_{}", tab_id)} role="tab" class="tab" aria-label="Code" />
                <div role="tabpanel" class="tab-content bg-base-100 border-base-300 rounded-box p-6">
                    <CodeBlock code=code />
                </div>
            </div>
        </div>
    }
}
