use toml::Value;
use crate::core::entities::player::Player;
use crate::models::commands::create_player_command::CreatePlayer;
use futures::TryFutureExt;
use log::info;
use crate::core::entities::party::Party;

pub async fn create_player_and_add_party_or_just_add_party(
    url: &String, party: &Party, name_player_content: &String
) -> Result<String, String> {
    player_exist(&url, &name_player_content)
        .and_then(|exist| switch_existing_player(exist, &url, &party, &name_player_content))
        .await
}

pub fn get_url_from_config() -> String {
    let contents = include_str!("../../../config.toml");
    let config: Value = toml::from_str(contents).expect("Could not parse TOML");
    let url = config["api"]["SUTOM_API_KEY"].as_str().expect("url chargement impossible");
    url.to_string()
}

async fn switch_existing_player(exist: bool, url: &String, party: &Party, name_player_content: &String) -> Result<String, String> {
    if exist.clone() {
        add_party_without_check(url, party, name_player_content)
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
                add_party_without_check(url, party, name_player_content)
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
        .map(|res| {
            info!("{}", res.clone());
            res
        })
        .map_err(|err| {
            info!("{}", err.clone());
            err
        })
}

async fn player_exist(url: &String, name: &String) -> Result<bool, String> {
    reqwest::get(format!("{}/players", url))
        .and_then(|response| {
            let body = response.json::<Vec<Player>>();
            body
        })
        .await
        .map(|players| {
            players
                .into_iter()
                .any(|player| {
                    player.name == name.clone()
                })
        })
        .map_err(|_| "une erreur est survenu".into())
}

async fn add_party_without_check(url: &String, party: &Party, name: &String) -> Result<u16, String> {
    reqwest::Client::new()
        .put(format!("{}/players/commands/add-party/{}", url, name))
        .json(&party)
        .send()
        .await
        .map(|response| response.status().as_u16())
        .map_err(|err| err.to_string())
}

async fn create_player_from_url(url: &String, name: &String) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/players/commands/create", url))
        .json(&CreatePlayer::new(name.clone()))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())
}
