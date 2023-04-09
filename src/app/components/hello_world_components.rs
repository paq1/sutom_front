use dioxus::prelude::*;

pub fn bandeau_smiley_component(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "smiley",
            "😀😁😂🤣😃😄😅😆😉😊😋😎😍😘🥰😗😙😚☺🙂🤗🤩🤔😌😛😜😝🤤😒😓😔😕🙃🤑😲☹🙁😖😞😟😤"
        }
    ))
}