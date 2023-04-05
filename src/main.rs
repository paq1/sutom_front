use std::string::ToString;

use dioxus::prelude::*;
use log::{info, LevelFilter};

use crate::app::components::hello_world_components::hello_world_component;
use crate::app::services::sutom_service_api_impl::SutomServiceApiImpl;

mod app;
mod core;
mod models;

fn main() {
    // launch the web app
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {

    let sutom_service_api = &SutomServiceApiImpl {
        url: "http://localhost:8000"
    };

    let name_player: &UseState<String> = use_state(cx, || "bob".to_string());
    let partie: &UseState<String> = use_state(cx, || "".to_string());

    let create_player = move |_| {
        let name_player_content = name_player.get().clone();
        cx.spawn({
            async move {
                info!("hello");
                sutom_service_api
                    .create(&name_player_content)
                    .await
                    .map(|_| info!("called"))
                    .map_err(|err| {
                        err
                    })
                    .expect("communication au service impossible");
            }
        });
    };

    cx.render(rsx! {
        style { include_str!("../src/style.css") }
        hello_world_component {}
        h1 {
            "SUTOM compétiton"
        }
        h2 {
            "Colle ton resultat de sutom"
        }
        input {
            value: "{name_player}",
            // and what to do when the value changes
            oninput: move |evt| name_player.set(evt.value.clone()),
        }
        p {
            "partie : "
            input {
                value: "{partie}",
                // and what to do when the value changes
                oninput: move |evt| partie.set(evt.value.clone()),
            }
        }
        p {
            "voici votre input : {name_player}"
        }
        button {
            onclick: create_player,
            "submit",
        }
    })
}