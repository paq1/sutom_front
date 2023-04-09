use std::string::ToString;

use dioxus::prelude::*;
use futures::TryFutureExt;
use log::{info, LevelFilter};

use crate::app::components::hello_world_components::hello_world_component;
use crate::app::services::party_handler_impl::PartyHandlerImpl;
use crate::app::services::sutom_service_api_impl::{add_party, create_player, create_player_from_url, get_url, player_exist};
use crate::core::entities::party::Party;
use crate::core::services::party_handler::PartyHandler;

mod app;
mod core;
mod models;

fn main() {
    // launch the web app
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

async fn traitement(exist: bool, url: &String, party: &Party, name_player_content: &String) -> Result<String, String> {
    if exist.clone() {
        add_party(url, party, name_player_content)
            .await
            .map(|res| {
                if res >= 400 {
                    String::from("vous avez déjà joué aujourd'hui")
                } else {
                    String::from("la partie a bien ete ajoutée 😘")
                }
            })
    } else {
        create_player_from_url(url, name_player_content)
            .and_then(|_| async move {
                add_party(url, party, name_player_content)
                    .and_then(|res| async move {
                        if res >= 400 {
                            Ok(String::from("vous avez déjà joué aujourd'hui"))
                        } else {
                            Ok(String::from("la partie a bien ete ajoutée 😘"))
                        }
                    })
                    .await
            })
            .await
    }
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

                player_exist(&url, &name_player_content)
                    .and_then(|exist| traitement(exist, &url, &party, &name_player_content))
                    .await
                    .expect("lors de l'ajout de partie");

                // match player_exist(&url, &name_player_content).await {
                //     Ok(exist) => traitement(exist, &url, &party, &name_player_content).await,
                //     Err(err) => Err(err)
                // }.expect("lors de l'ajout de partie");
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