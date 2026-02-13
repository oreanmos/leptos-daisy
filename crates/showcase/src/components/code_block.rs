use leptos::prelude::*;

use super::syntax::highlight_code;

/// A code block component with Rust syntax highlighting and copy-to-clipboard.
/// Uses syntect for native Rust-based syntax highlighting.
#[component]
pub fn CodeBlock(
    /// The code content to display
    #[prop(into)]
    code: String,
    /// Language extension/token for syntax highlighting (rs, css, html, etc.)
    #[prop(optional, into)]
    language: Option<String>,
    /// Optional title/header for the code block
    #[prop(optional, into)]
    title: Option<String>,
    /// Optional class for the container
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let copied = RwSignal::new(false);
    let code_for_copy = code.clone();

    let language = language.unwrap_or_else(|| "rs".to_string());
    let highlighted = highlight_code(&code, &language);

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
                <pre class="pr-20"><code inner_html=highlighted></code></pre>
            </div>
        </div>
    }
}
