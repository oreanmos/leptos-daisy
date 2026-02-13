use leptos::prelude::*;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use wasm_bindgen::JsCast;

/// A code block component with syntax highlighting and copy-to-clipboard.
/// Uses highlight.js (loaded via CDN in index.html) for Rust syntax highlighting.
#[component]
pub fn CodeBlock(
    /// The code content to display
    #[prop(into)]
    code: String,
    /// Optional title/header for the code block
    #[prop(optional, into)]
    title: Option<String>,
    /// Optional class for the container
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let copied = RwSignal::new(false);
    let code_for_copy = code.clone();
    let code_ref = NodeRef::<leptos::html::Code>::new();

    let copy_to_clipboard = move |_| {
        let code = code_for_copy.clone();
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            use web_sys::window;
            if let Some(window) = window() {
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&code);
                copied.set(true);
                let copied_signal = copied;
                set_timeout(
                    move || copied_signal.set(false),
                    std::time::Duration::from_secs(2),
                );
            }
        }
        #[cfg(not(any(feature = "csr", feature = "hydrate")))]
        {
            let _ = code;
            let _ = copied;
        }
    };

    // Apply syntax highlighting after the element is mounted
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let code_ref_effect = code_ref;
        Effect::new(move |_| {
            if let Some(el) = code_ref_effect.get() {
                let el: &web_sys::HtmlElement = &el;
                let hljs = js_sys::Reflect::get(&web_sys::window().unwrap(), &"hljs".into());
                if let Ok(hljs) = hljs
                    && !hljs.is_undefined()
                {
                    let highlight_fn = js_sys::Reflect::get(&hljs, &"highlightElement".into());
                    if let Ok(func) = highlight_fn
                        && let Some(func) = func.dyn_ref::<js_sys::Function>()
                    {
                        let _ = func.call1(&hljs, el);
                    }
                }
            }
        });
    }

    view! {
        <div class=move || class.get().unwrap_or_default()>
            {move || title.clone().map(|t| view! {
                <div class="text-sm font-mono text-base-content/70 mb-2">{t}</div>
            })}
            <div class="mockup-code w-full max-w-full overflow-x-auto relative">
                <button
                    type="button"
                    class="btn btn-sm btn-ghost absolute top-2 right-2 z-10"
                    on:click=copy_to_clipboard
                >
                    {move || if copied.get() {
                        view! { <span class="text-success">"Copied!"</span> }.into_any()
                    } else {
                        view! { <span>"Copy"</span> }.into_any()
                    }}
                </button>
                <pre class="pr-20"><code class="language-rust" node_ref=code_ref>{code}</code></pre>
            </div>
        </div>
    }
}
