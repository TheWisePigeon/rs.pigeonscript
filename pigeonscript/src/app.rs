use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::header::Header;
use crate::components::header::HeaderProps;
use crate::pages::{ home::Home, home::HomeProps };


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link href="https://fonts.googleapis.com/css2?family=Oswald:wght@200;300;400;500;600;700&family=Oxygen:wght@300;400;700&display=swap" rel="stylesheet"/>
        // sets the document title
        <Title text="PigeonScript"/>

        // content for this welcome page
        <Router>
            <main>
                <Header />
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Home/> }/>
                </Routes>
            </main>
        </Router>
    }
}
