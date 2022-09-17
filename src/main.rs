use std::collections::HashMap;

mod trello;

#[async_std::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read dotenv file");

    let key = dotenv::var("TRELLO_API_KEY").expect("no TRELLO_API_KEY set");
    let token = dotenv::var("TRELLO_API_TOKEN").expect("no TRELLO_API_TOKEN set");
    let auth = trello::AuthenticationPair {
        key: &key,
        token: &token,
    };

    let main_board = trello::Board("5dc623860361f511f891bbcd");

    let lists = main_board
        .lists(&auth)
        .await
        .expect("Failed to query lists")
        .into_iter()
        .map(|l| (l.id, l.name))
        .collect::<HashMap<_, _>>();

    let cards = main_board
        .cards(&auth)
        .await
        .expect("Failed to query cards");
    for c in cards {
        println!("{}({})", c.name, lists[&c.id_list]);
    }
}
