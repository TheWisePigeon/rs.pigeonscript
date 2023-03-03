use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView{
    view!{
        cx,
        <main>
            <h1>{"Welcome to PigeonScript boi"}</h1>
        </main>
    }
}