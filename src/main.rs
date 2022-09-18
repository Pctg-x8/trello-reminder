use std::borrow::Cow;
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
    pub pos: f32,
    pub cards: Vec<trello::BoardCard>,
}

async fn run(_: LambdaEvent<serde_json::Value>) -> Result<(), lambda_runtime::Error> {
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
                    pos: l.pos,
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
    let mut order = cards_by_list
        .into_values()
        .filter(|c| c.name != "完了")
        .collect::<Vec<_>>();
    order.sort_by(|a, b| a.pos.partial_cmp(&b.pos).expect("nan pos"));

    let mut blocks = Vec::with_capacity(1 + order.len());
    blocks.push(async_slack_web_api::Block::Section {
        text: Some(async_slack_web_api::TextObject::Markdown {
            text: Cow::Borrowed("今Trelloにあるタスクはこんな感じだよ！"),
            verbatim: None,
        }),
        block_id: None,
        fields: Vec::new(),
    });
    let mut text = String::from("\n\n");
    blocks.extend(order.into_iter().map(|c| {
        let mut text = format!("*{}*\n", c.name);
        for cc in &c.cards {
            text.extend(
                format!(
                    "<{}|{}>({})\n",
                    cc.name,
                    cc.url,
                    cc.labels
                        .iter()
                        .map(|l| &l.name as &str)
                        .collect::<Vec<_>>()
                        .join(" / ")
                )
                .chars(),
            );
        }

        async_slack_web_api::Block::Section {
            text: Some(async_slack_web_api::TextObject::Markdown {
                text: Cow::Owned(text),
                verbatim: None,
            }),
            block_id: None,
            fields: Vec::new(),
        }
    }));

    async_slack_web_api::api::chat::PostMessage
        .send(
            &secrets.slack_api_token,
            async_slack_web_api::api::chat::post_message::Request {
                channel: POST_CHANNEL_ID,
                blocks: Some(&serde_json::to_string(&blocks)?),
                ..Default::default()
            },
        )
        .await?
        .into_result()?;

    Ok(())
}
