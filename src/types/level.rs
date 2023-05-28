use super::terminal_content::TerminalContent;
use leptos::{html::AnyElement, HtmlElement};

#[derive(Clone)]
pub struct Level {
    pub content: TerminalContent,
    pub instructions: HtmlElement<AnyElement>,
}
