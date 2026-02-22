use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

/// Highlight code in the requested language and return HTML with inline styles.
pub fn highlight_code(code: &str, language: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss
        .find_syntax_by_extension(language)
        .or_else(|| ss.find_syntax_by_token(language))
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let theme = &ts.themes["base16-ocean.dark"];
    highlighted_html_for_string(code, &ss, syntax, theme).unwrap_or_else(|_| escape_html(code))
}

/// Highlight Rust code and return HTML with inline styles.
pub fn highlight_rust(code: &str) -> String {
    highlight_code(code, "rs")
}
