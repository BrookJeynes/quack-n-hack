use leptos::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="bg-matte-black text-white">
            <div class="lg:max-w-header mx-auto px-5 py-5">
                <h1 class="text-2xl">"$ Terminal"<span class="text-mint-green">"Froggy"</span></h1>
                <div id="divider" class="h-0.5 mt-3 bg-white opacity-20 mx-auto" />
            </div>
        </div>
    }
}
