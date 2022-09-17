#[derive(serde::Serialize)]
pub struct AuthenticationPair<'s> {
    pub key: &'s str,
    pub token: &'s str,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardCardLabel {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardCard {
    pub id: String,
    pub name: String,
    pub url: String,
    pub desc: String,
    #[serde(default)]
    pub labels: Vec<BoardCardLabel>,
    pub id_list: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardList {
    pub id: String,
    pub name: String,
}

pub struct Board<'s>(pub &'s str);
impl Board<'_> {
    pub async fn cards(&self, auth: &AuthenticationPair<'_>) -> reqwest::Result<Vec<BoardCard>> {
        reqwest::Client::new()
            .get(format!("https://api.trello.com/1/boards/{}/cards", self.0))
            .query(auth)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn lists(&self, auth: &AuthenticationPair<'_>) -> reqwest::Result<Vec<BoardList>> {
        reqwest::Client::new()
            .get(format!("https://api.trello.com/1/boards/{}/lists", self.0))
            .query(auth)
            .send()
            .await?
            .json()
            .await
    }
}
