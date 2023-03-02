use leptos::*;
use crate::components::Test;
use crate::components::TestProps;

#[component]
pub fn Posts(cx: Scope) -> impl IntoView{
    view! {
        cx,
        <main>
            <Test />
        </main>
    }
}