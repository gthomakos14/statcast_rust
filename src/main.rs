mod api_handler;

#[tokio::main]
async fn main() {
    let date = "2023-12-30".to_owned();
    let game_pks = api_handler::get_game_pks(date).await;
    let games_length = game_pks.len();
    println!("{games_length}");
    for game_pk in game_pks{
        println!("{game_pk}");
    }
}


