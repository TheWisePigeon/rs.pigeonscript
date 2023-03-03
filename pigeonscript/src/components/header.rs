use leptos::*;

#[component]
pub fn header(cx: Scope) -> impl IntoView{
    view!{
        cx,
        <main class="header" >
            <a class="is_current_url" href="/" >{"PigeonScript"}</a>
            <a class="is_current_url" href="/issues">{"issues"}</a>
        </main>
    }
}