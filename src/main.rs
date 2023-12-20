use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App)
}

#[component]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        input {
            r#type: "search",
        },
        button {
            prevent_default: "onmousedown",
            r#type: "button",
            "click",
        }
    })
}
