use std::string::ToString;

use dioxus::prelude::*;
use log::{LevelFilter};

use crate::app::components::hello_world_components::hello_world_component;
use crate::app::services::party_handler_impl::PartyHandlerImpl;
use crate::app::services::sutom_service_api_impl::{get_url, create_player_and_add_party_or_just_add_party};
use crate::core::services::party_handler::PartyHandler;

mod app;
mod core;
mod models;

fn main() {
    // launch the web app
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {

    let name_player: &UseState<String> = use_state(cx, || "bob".to_string());
    let partie_state: &UseState<String> = use_state(cx, || "".to_string());

    let add_party_player = move |_| {
        let name_player_content = name_player.get().clone();
        let partie_content = partie_state.get().clone();
        let url = get_url();
        let party = PartyHandlerImpl::handle_message(&partie_content).expect("error parsing");
        cx.spawn({
            async move {
                create_player_and_add_party_or_just_add_party(&url, &party, &name_player_content)
                    .await
                    .expect("erreur lors de l'ajout de partie");
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
            textarea {
                value: "{partie_state}",
                // and what to do when the value changes
                oninput: move |evt| partie_state.set(evt.value.clone()),
            }
        }
        p {
            "voici votre input : {name_player}"
        }
        button {
            onclick: add_party_player,
            "submit",
        }
    })
}