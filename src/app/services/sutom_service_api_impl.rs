use crate::models::commands::create_player_command::CreatePlayer;

#[derive(Clone)]
pub struct SutomServiceApiImpl {
    pub url: &'static str
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
