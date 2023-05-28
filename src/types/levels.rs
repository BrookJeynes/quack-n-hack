use super::level::Level;
use super::terminal_content::TerminalContent;
use leptos::{view, Scope};

pub struct Levels {
    pub levels: Vec<Level>,
}

impl Levels {
    pub fn init(cx: Scope) -> Levels {
        Self {
            levels:vec![
                Level {
                    content: TerminalContent::new(
                        "lesson-01_baby-steps", 
                        vec![
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
                        ]
                    ),
                    instructions: view! {cx,
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
                    }.into_any()
                },

                Level {
                    content: TerminalContent::new(
                        "lesson-02_cats", 
                        vec![
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
                            (String::from("cat Quacky.duck"), view! {cx, 
                                <div>
                                    <p><span class="text-pastel-blue">"❯ "</span>"cat "<span class="text-pastel-yellow">"Quacky.duck"</span></p>
                                    <pre class="text-pastel-yellow">r#"
                                             _
                                          __(.)<
                                      ... \___)
                                    "#</pre>
                                </div>
                            }.into_any()),
                        ]
                    ),
                    instructions: view! {cx,
                        <div class="flex flex-col gap-5">
                            <p>
                                "Cats?"
                            </p>
                        </div>
                    }.into_any()
                }
            ]
        }
    }

    pub fn get_level(&self, level: usize) -> Option<Level> {
        self.levels.get(level).cloned()
    }
}
