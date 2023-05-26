use leptos::*;

use crate::components::page_wrapper::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <PageWrapper>
            <p>"Hello World"</p>
        </PageWrapper>
    }
}
