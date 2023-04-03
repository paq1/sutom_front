use dioxus::html::{input, style};
use dioxus::prelude::*;
use serde::{
    Serialize, Deserialize
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}

impl CreatePlayer {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {

    let name: &UseState<String> = use_state(cx, || "bob".to_string());
    let partie: &UseState<String> = use_state(cx, || "".to_string());
    let url = "http://localhost:8000";


    let create_player_future = use_future(cx, (), |_| async move {
        // name.set("toto".to_string());
        println!("lol")
    });

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
            "partie : "
        }
        input {
            value: "{partie}",
            // and what to do when the value changes
            oninput: move |evt| partie.set(evt.value.clone()),
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
            onclick: |_| println!("ok"),// create_player_future.restart(),
            "submit",
        }
    })
}