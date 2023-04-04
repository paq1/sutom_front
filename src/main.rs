use dioxus::html::{input, style};
use dioxus::prelude::*;
use serde::{
    Serialize, Deserialize
};
use log::{LevelFilter, info};

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
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

async fn test_async() {
    info!("hello");
    let response = reqwest::Client::new()
        .post("https://example.com/login")
        .fetch_mode_no_cors()
        .json(&CreatePlayer::new("mon giga test".to_string()))
        .send()
        .await;
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {

    let name_player: &UseState<String> = use_state(cx, || "bob".to_string());
    let partie: &UseState<String> = use_state(cx, || "".to_string());
    let url = "http://localhost:8000";

    let create_player = move |_| {

        // let val_player = name_player.get().clone();

        cx.spawn({
            async move {

                test_async().await;
                // let response = reqwest::Client::new()
                //     .post("http://localhost:8000/players/commands/create")
                //     .fetch_mode_no_cors()
                //     .json(&CreatePlayer::new("toto".to_string()))
                //     .send()
                //     .await;

                // match response {
                //     Ok(data) => {
                //         println!("created!");
                //         // logged_in.set(true);
                //     }
                //     Err(err) => {
                //         println!(
                //             "not created"
                //         )
                //     }
                // }
            }
        });
    };

    cx.render(rsx! {
        style { include_str!("../src/style.css") }
        h1 {
            "SUTOM compétiton"
        }
        h2 {
            "colle ton resultat de sutom"
        }
        input {
            value: "{name_player}",
            // and what to do when the value changes
            oninput: move |evt| name_player.set(evt.value.clone()),
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
        p {"{name_player}"}
        div {
            class: "helloWorld",
            "Hello, world!!!!"
        }
        button {
            onclick: create_player,
            "submit",
        }
    })
}