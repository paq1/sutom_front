use toml::Value;
use crate::core::entities::player::Player;
use crate::models::commands::create_player_command::CreatePlayer;
use futures::TryFutureExt;
use crate::core::entities::party::Party;

pub async fn player_exist(url: &String, name: &String) -> Result<bool, String> {
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

pub async fn add_party(url: &String, party: &Party, name: &String) -> Result<u16, String> {
    reqwest::Client::new()
        .put(format!("{}/players/commands/add-party/{}", url, name))
        .json(&party)
        .send()
        .await
        .map(|response| response.status().as_u16())
        .map_err(|err| err.to_string())
}

pub async fn create_player(name: &String) -> Result<(), String> {
    let contents = include_str!("../../../config.toml");
    let config: Value = toml::from_str(contents).expect("Could not parse TOML");
    let url = config["api"]["SUTOM_API_KEY"].as_str().expect("url chargement impossible");
    create_player_from_url(&url.to_string(), name).await
}

pub async fn create_player_from_url(url: &String, name: &String) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/players/commands/create", url))
        .json(&CreatePlayer::new(name.clone()))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())
}

pub fn get_url() -> String {
    let contents = include_str!("../../../config.toml");
    let config: Value = toml::from_str(contents).expect("Could not parse TOML");
    let url = config["api"]["SUTOM_API_KEY"].as_str().expect("url chargement impossible");
    url.to_string()
}