use crate::components::page_wrapper::*;
use crate::components::terminal::*;
use crate::types::terminal_content::TerminalContent;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Signals
    let (complete, set_complete) = create_signal(cx, false);
    // TODO - Temporary: Parse this from a file instead of hard-coding
    let terminal = TerminalContent::new("lesson-01_baby-steps", vec![
        (String::from("ls"), view! {cx, 
            <div>
                <p><span class="text-pastel-blue">"❯ "</span>"ls"</p>
                <div class="flex justify-around">
                    <span>"Waddle.duck"</span>
                    <span>"Ducky.duck"</span>
                    <span>"Pippin.duck"</span>
                    <a class="cursor-pointer"><span class="text-pastel-yellow">"Quacky.duck"</span></a>
                    <span>"Bubbles.duck"</span>
                </div>
            </div>
        }.into_any()),
        (String::from("help"), view! {cx, <p><span class="text-pastel-blue">"❯ "</span>"help"</p>}.into_any()),
    ]);

    // Effects
    create_effect(cx, move |_| {
        complete();
        let _ = js_sys::eval(
            "window.confetti({
                    particleCount: 410,
                    spread: 100,
                    origin: { y: 0.6 }
                });",
        );
    });

    view! { cx,
        <PageWrapper>
            <Terminal terminal={terminal} complete_callback={set_complete} />
        </PageWrapper>
    }
}
