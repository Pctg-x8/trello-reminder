use std::collections::HashMap;

use lambda_runtime::{service_fn, LambdaEvent};
use secrets::Secrets;

mod secrets;
mod trello;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(service_fn(run)).await
}

const MAIN_BOARD: trello::Board = trello::Board("5dc623860361f511f891bbcd");
const POST_CHANNEL_ID: &'static str = "CQDSQHCTZ";

pub struct List {
    pub name: String,
    pub cards: Vec<trello::BoardCard>,
}

async fn run(_: LambdaEvent<()>) -> Result<(), lambda_runtime::Error> {
    let secrets = Secrets::load().await?;
    let auth = trello::AuthenticationPair {
        key: &secrets.trello_api_key,
        token: &secrets.trello_api_token,
    };

    let (lists, cards) = tokio::try_join!(MAIN_BOARD.lists(&auth), MAIN_BOARD.cards(&auth))?;
    let mut cards_by_list = lists
        .into_iter()
        .map(|l| {
            (
                l.id,
                List {
                    name: l.name,
                    cards: Vec::new(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    for c in cards {
        cards_by_list
            .get_mut(&c.id_list)
            .expect("unknown list id")
            .cards
            .push(c);
    }

    let mut text = String::from("今Trelloにあるタスクはこんな感じだよ！\n\n");
    for c in cards_by_list.values() {
        text.extend(format!("* {}\n", c.name).chars());
        for cc in &c.cards {
            text.extend(
                format!(
                    "  * {}({})\n",
                    cc.name,
                    cc.labels
                        .iter()
                        .map(|l| &l.name as &str)
                        .collect::<Vec<_>>()
                        .join(" / ")
                )
                .chars(),
            );
        }
    }

    async_slack_web_api::api::chat::PostMessage
        .send(
            &secrets.slack_api_token,
            async_slack_web_api::api::chat::post_message::Request {
                channel: POST_CHANNEL_ID,
                text: Some(&text),
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}
