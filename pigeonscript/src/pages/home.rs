use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView{
    view!{
        cx,
        <main class="home">
            <div>
                <h1 class="gradient-text" >{"PIGEONSCRIPT IS FUN AND INFORMATIVE"}</h1>
                <p>
                    {"Stay up-to-date on the latest web development news and trends, and learn new skills with tutorials and tips, all delivered with a healthy dose of humour"}
                </p>
            </div>
            <form class="subscribe" >
                <input  type="text" placeholder="Sup niggae" />
                <button class="sub_button" >
                    {"Sign me up! "}
                </button>
            </form>
        </main>
    }
}