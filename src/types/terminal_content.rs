use crate::{html::AnyElement, view, HtmlElement, IntoView, Scope};

#[derive(Clone)]
pub struct TerminalContent {
    progress: usize,
    pub title: String,
    pub answers: Vec<(String, HtmlElement<AnyElement>)>,
}

impl TerminalContent {
    pub fn new(title: &str, answers: Vec<(String, HtmlElement<AnyElement>)>) -> Self {
        TerminalContent {
            progress: 0,
            title: title.to_string(),
            answers,
        }
    }

    pub fn create_title(&self, cx: Scope) -> impl IntoView {
        view! {cx,
            <p>
                <span class="text-pastel-blue">"❯ "</span>{self.title.clone()}
            </p>
        }
    }

    pub fn check_answer(&mut self, input: &str) -> Result<HtmlElement<AnyElement>, ()> {
        let output = self
            .answers
            .get(self.progress)
            .expect("self.progress to never provide an out of bounds value");

        if output.0 == input {
            Ok(output.1.clone())
        } else {
            Err(())
        }
    }

    pub fn next(&mut self) -> Option<()> {
        if self.check_finished() {
            return None;
        }

        self.progress += 1;
        Some(())
    }

    pub fn check_finished(&self) -> bool {
        self.progress == self.answers.len() - 1
    }
}
