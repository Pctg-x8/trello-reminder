use std::collections::HashMap;

use lambda_runtime::{service_fn, LambdaEvent};
use secrets::Secrets;

mod secrets;
mod trello;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    dotenv::dotenv().expect("Failed to read dotenv file");

    lambda_runtime::run(service_fn(run)).await
}

const MAIN_BOARD: trello::Board = trello::Board("5dc623860361f511f891bbcd");

async fn run(_: LambdaEvent<()>) -> Result<(), lambda_runtime::Error> {
    let secrets = Secrets::load().await?;
    let auth = trello::AuthenticationPair {
        key: &secrets.trello_api_key,
        token: &secrets.trello_api_token,
    };

    let (lists, cards) = tokio::try_join!(MAIN_BOARD.lists(&auth), MAIN_BOARD.cards(&auth))?;
    let lists = lists
        .into_iter()
        .map(|l| (l.id, l.name))
        .collect::<HashMap<_, _>>();
    for c in cards {
        println!("{}({})", c.name, lists[&c.id_list]);
    }

    Ok(())
}
