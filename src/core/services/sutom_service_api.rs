use async_trait::async_trait;

#[async_trait]
pub trait SutomServiceApi {
    async fn create_account(&self, name: String) -> Result<(), String>;
    // async fn add_party(&self, party: Party, name: String) -> Result<u16, String>;
}