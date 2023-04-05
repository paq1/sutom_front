use dioxus::prelude::*;

pub fn hello_world_component(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "helloWorld",
            "Hello, world!!!!"
        }
    ))
}