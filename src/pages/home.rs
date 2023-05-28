use crate::components::instructions_panel::*;
use crate::components::page_wrapper::*;
use crate::components::terminal::*;
use crate::types::levels::Levels;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Variables
    let levels = Levels::init(cx);
    let level_len = levels.levels.len() - 1;

    // Signals
    let (complete, set_complete) = create_signal(cx, false);
    let (level, set_level) = create_signal(cx, 0);
    let terminal = Signal::derive(cx, move || {
        levels
            .get_level(level())
            .expect("there to always be a valid level")
    });

    // Callbacks
    let increment_level = move || {
        if level() < level_len {
            set_level.update(|n| *n += 1);
        }
    };

    let decrement_level = move || {
        if level() > 0 {
            set_level.update(|n| *n -= 1);
        }
    };

    // Effects
    create_effect(cx, move |_| {
        if complete() {
            let _ = js_sys::eval(
                "window.confetti({
                    particleCount: 650,
                    spread: 100,
                    origin: { y: 0.6 }
                });",
            );
        }
    });

    view! { cx,
        <PageWrapper>
            <div class="flex flex-col md:flex-row justify-around gap-10">
                <InstructionsPanel
                    class=String::from("w-full md:w-1/3")
                    increment_level_callback={increment_level}
                    decrement_level_callback={decrement_level}
                    level={level}
                >
                    {move || terminal().instructions}
                </InstructionsPanel>
                <Terminal
                    class=String::from("w-full md:w-2/3")
                    terminal={terminal}
                    complete_callback={set_complete}
                />
            </div>
        </PageWrapper>
    }
}
