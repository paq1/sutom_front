use dioxus::prelude::Props;
use crate::models::commands::create_player_command::CreatePlayer;

#[derive(Clone, Props, PartialEq)]
pub struct SutomServiceApiImpl<'a> {
    pub url: &'a str
}

impl SutomServiceApiImpl<'_> {
    pub async fn create(&self, name: &String) -> Result<(), String> {
        // fixme Uncaught (in promise) Error: url parse
        reqwest::Client::new()
            .post(format!("{}/players/commands/create", self.url))
            .json(&CreatePlayer::new(name.clone()))
            .send()
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}
