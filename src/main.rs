#![allow(non_snake_case)]

use dioxus::html::{input, style};
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let name = use_state(cx, || "bob".to_string());

    cx.render(rsx! {
        style { include_str!("../src/style.css") }
        h1 {
            "SUTOM compétiton"
        }
        h2 {
            "colle ton resultat de sutom"
        }
        input {
            value: "{name}",
            // and what to do when the value changes
            oninput: move |evt| name.set(evt.value.clone()),
        }
        p {
            "voici votre input"
        }
        br {}
        p {"{name}"}
        div {
            class: "helloWorld",
            "Hello, world!!!!"
        }
        button {
            onclick: move |evt| name.set("coucou".into()),
            "submit",
        }
    })
}