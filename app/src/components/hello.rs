use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct MfName{
    pub name: String,
    pub age: String
}

#[function_component]
pub fn Hello( props: &MfName ) -> Html{
    html!{
        <main>
            <h1>{"Hi, I'm the mfing mf. Call me "}{ props.name.clone() }{ "And I'm " } { props.age.clone() } </h1>
        </main>
    }
}