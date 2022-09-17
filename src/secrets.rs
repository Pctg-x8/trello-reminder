use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};

#[derive(serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Secrets {
    pub trello_api_key: String,
    pub trello_api_token: String,
    pub slack_api_token: String,
}
impl Secrets {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let secret_string = SecretsManagerClient::new(Region::ApNortheast1)
            .get_secret_value(GetSecretValueRequest {
                secret_id: String::from("trello-reminder"),
                ..Default::default()
            })
            .await?
            .secret_string
            .expect("no secret string?");

        serde_json::from_str(&secret_string).map_err(From::from)
    }
}
