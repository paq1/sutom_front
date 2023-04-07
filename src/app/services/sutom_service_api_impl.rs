use dioxus::prelude::Props;
use toml::Value;
use crate::models::commands::create_player_command::CreatePlayer;

pub async fn create_player(name: &String) -> Result<(), String> {
    let contents = include_str!("../../../config.toml");
    let config: Value = toml::from_str(contents).expect("Could not parse TOML");
    let url = config["api"]["SUTOM_API_KEY"].as_str().expect("url chargement impossible");
    create_player_from_url(url, name).await
}

pub async fn create_player_from_url(url: &str, name: &String) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/players/commands/create", url))
        .json(&CreatePlayer::new(name.clone()))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| err.to_string())
}