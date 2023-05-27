use crate::components::instructions_panel::*;
use crate::components::page_wrapper::*;
use crate::components::terminal::*;
use crate::types::terminal_content::TerminalContent;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // Signals
    let (complete, set_complete) = create_signal(cx, false);
    let (level, set_level) = create_signal(cx, 0);
    // TODO - Temporary: Parse this from a file instead of hard-coding
    let terminal = TerminalContent::new("lesson-01_baby-steps", vec![
        (String::from("ls"), view! {cx, 
            <div>
                <p><span class="text-pastel-blue">"❯ "</span>"ls"</p>
                <div class="flex justify-around flex-wrap gap-2">
                    <span>"Waddle.duck"</span>
                    <span>"Ducky.duck"</span>
                    <span>"Pippin.duck"</span>
                    <a class="cursor-pointer"><span class="text-pastel-yellow">"Quacky.duck"</span></a>
                    <span>"Bubbles.duck"</span>
                </div>
            </div>
        }.into_any()),
    ]);

    // Callbacks
    let increment_level = move || {
        set_level(level() + 1);
    };

    let decrement_level = move || {
        if level() > 0 {
            set_level(level() - 1);
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
                    increment_level_callback=increment_level
                    decrement_level_callback=decrement_level
                    level=level
                >
                    <div class="flex flex-col gap-5">
                        <p>
                            "Welcome to "<span class="text-pastel-yellow">"Quack"
                            </span>" ‘n Hack, a game where you go through a series
                            of challenges helping "<span class="text-pastel-yellow">
                            "Quacky"</span>" find his friends."
                        </p>
                        <p>
                            "To help find "<span class="text-pastel-yellow">"Quacky's"
                            </span>" friends you’ll have to navigate the filesystem
                            using "<span class="text-pastel-green">"Unix"</span>" 
                            commands, along the way learning new tools."
                        </p>
                        <p>
                            "Speaking of "<span class="text-pastel-yellow">"Quacky"
                            </span>", where is he?"
                        </p>
                        <p>
                            "The "<span class="text-pastel-purple">"`ls`"</span>
                            " command will print all items in your current working
                            "<span class="text-pastel-green">"directory"</span>" 
                            in the terminal."</p>
                        <p>
                            "Try typing the command in the terminal to our right."
                        </p>
                    </div>
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
