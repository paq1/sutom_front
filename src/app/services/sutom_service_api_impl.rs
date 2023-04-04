use crate::core::services::sutom_service_api::SutomServiceApi;
use crate::models::commands::create_player_command::CreatePlayer;

use async_trait::async_trait;

#[derive(Clone)]
pub struct SutomServiceApiImpl {
    pub url: String
}

impl SutomServiceApiImpl {
    pub async fn create(&self, name: &String) -> Result<(), String> {
        // Ok(())
        // fixme Uncaught (in promise) Error: url parse
        reqwest::Client::new()
            .post(format!("{}/players/commands/create", self.url))
            .fetch_mode_no_cors()
            .json(&CreatePlayer::new(name.clone()))
            .send()
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}
