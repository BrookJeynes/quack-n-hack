use leptos::*;

#[component]
pub fn PageWrapper(cx: Scope, children: Children) -> impl IntoView {
    view! {
        cx,
        <div class="h-full min-h-screen bg-matte-black text-white">
            <div class="pt-5 md:pt-20 pb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-content">
                {children(cx)}
            </div>
        </div>
    }
}
