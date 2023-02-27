use yew::prelude::*;
use crate::components::hello::Hello;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <Hello name={"Mfer"} age={"15"} />
        </main>
    }
}
