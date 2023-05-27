use leptos::*;

#[component]
pub fn InstructionsPanel<F, V>(
    cx: Scope,
    children: Children,
    #[prop(optional)] class: String,
    increment_level_callback: F,
    decrement_level_callback: V,
    level: ReadSignal<usize>,
) -> impl IntoView
where
    F: Fn() + 'static,
    V: Fn() + 'static,
{
    view! {
        cx,
        <div
            class=format!("{} {} {}",
                "bg-terminal-dark-blue text-white border-2 border-pastel-purple",
                "h-96 md:h-[30rem] flex flex-col justify-between",
                class)
        >
            <div class="overflow-scroll mx-auto p-5">
                {children(cx)}
            </div>

            <div class="flex flex-row justify-around items-center px-4 py-2">
                <svg
                    class="w-7 fill-white cursor-pointer" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"
                    on:click=move |_| decrement_level_callback()
                >
                    <path d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l160 160c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L109.2 288 416 288c17.7 0 32-14.3 32-32s-14.3-32-32-32l-306.7 0L214.6 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-160 160z"/>
                </svg>

                <span>"Level "{level}</span>

                <svg
                    class="w-7 fill-white cursor-pointer" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"
                    on:click=move |_| increment_level_callback()
                >
                    <path d="M438.6 278.6c12.5-12.5 12.5-32.8 0-45.3l-160-160c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L338.8 224 32 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l306.7 0L233.4 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l160-160z"/>
                </svg>
            </div>
        </div>
    }
}