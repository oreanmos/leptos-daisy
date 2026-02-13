use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Highlight code in the requested language and return HTML with inline styles.
pub fn highlight_code(code: &str, language: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss
        .find_syntax_by_extension(language)
        .or_else(|| ss.find_syntax_by_token(language))
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let theme = &ts.themes["base16-ocean.dark"];
    highlighted_html_for_string(code, &ss, syntax, theme).unwrap_or_else(|_| code.to_string())
}

/// Highlight Rust code and return HTML with inline styles.
pub fn highlight_rust(code: &str) -> String {
    highlight_code(code, "rs")
}
