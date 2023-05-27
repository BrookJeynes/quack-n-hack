use leptos::*;

#[component]
pub fn Tooltip(cx: Scope, children: Children, text: String) -> impl IntoView {
    view! {
        cx,
        <span class="group relative w-max">
            {children(cx)}
            <span
                class=format!("{} {} {}",
                    "pointer-events-none absolute -top-7 left-0 w-max",
                    "opacity-0 transition-opacity group-hover:opacity-100",
                    "bg-pastel-green rounded p-2"
                )
            >
                {text}
            </span>
        </span>
        // </span>
    }
}
