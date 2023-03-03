use leptos::*;

#[component]
pub fn header(cx: Scope) -> impl IntoView{
    view!{
        cx,
        <main class="header" >
            <a href="/" >{"PigeonScript"}</a>
            <a href="/issues">{"issues"}</a>
        </main>
    }
}