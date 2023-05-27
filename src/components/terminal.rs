use crate::types::terminal_content::TerminalContent;
use leptos::{ev::SubmitEvent, html::Input, *};
use std::time::Duration;

// TODO - Tech Debt: Find a way to not allow the whole 
// component but rather the single closure
#[allow(clippy::redundant_closure)]
#[component]
pub fn Terminal<F>(
    cx: Scope,
    terminal: TerminalContent,
    /// Callback used to signal when the terminal instance is completed
    complete_callback: F,
    #[prop(optional)] class: String,
) -> impl IntoView
where
    F: Fn(bool) + 'static,
{
    // Variables
    let mut terminal = terminal;
    let title = terminal.create_title(cx);

    // References
    let terminal_input_ref: NodeRef<Input> = create_node_ref(cx);

    // Signals
    let (show_error, set_show_error) = create_signal(cx, false);
    let (disable_input, set_disable_input) = create_signal(cx, false);
    let (terminal_content, set_terminal_content) = create_signal(cx, Vec::new());

    // Handlers
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let input = terminal_input_ref().expect("<input> to exist").value();

        match terminal.check_answer(input.trim()) {
            Ok(output) => {
                set_terminal_content.update(|content| content.push(output));

                if terminal.next().is_none() {
                    set_disable_input(true);
                    complete_callback(true);
                }
            }
            Err(_) => {
                if !show_error() {
                    set_show_error(true);
                }
            }
        }

        terminal_input_ref()
            .expect("<input> to exist")
            .set_value("");
    };

    // Effects
    create_effect(cx, move |_| {
        if show_error() {
            set_timeout(move || set_show_error(false), Duration::new(1, 0));
        }
    });

    // Classes
    let body_styling = format!("bg-terminal-dark-blue text-white border-2 border-pastel-purple h-[30rem] overflow-scroll {}", class);

    view! {
        cx,
        <div
            class=move || if show_error() {format!("{} animate-shake", body_styling)} else {body_styling.to_string()}
            on:click=move |_| terminal_input_ref().expect("<input> to exist").focus().expect("<input> to exist")
        >
            <div class="mx-auto p-5">
                <form on:submit=on_submit>
                    <p>{title}</p>

                    {terminal_content}

                    <p>
                        "[root@"<span class="text-pastel-blue">"quack"</span>" ~]$ "
                            <input
                                class="outline-none bg-transparent"
                                type="text"
                                node_ref=terminal_input_ref
                                // Closure not redundant - required for state updates
                                disabled=move || disable_input()
                                autofocus
                            />
                    </p>
                </form>
            </div>
        </div>
    }
}
