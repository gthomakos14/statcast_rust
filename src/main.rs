mod modules;

#[tokio::main]
async fn main() {
    // let date = "2023-06-30".to_owned();
    // print_game_pks(date);
    let yankees_pk = "717550";
    let mut df_box = modules::data_formatter::get_box_data(yankees_pk)
        .await;
    println!("{}", &df_box);
    let mut file = std::fs::File::create("yankees_wedding_box.parquet").unwrap();
    polars::prelude::ParquetWriter::new(&mut file).finish(&mut df_box).unwrap();
}


async fn print_game_pks(date: String){
    let game_pks = modules::api_handler::get_game_pks(date).await;
    let games_length = game_pks.len();
    println!("{games_length}");
    for game_pk in game_pks{
        println!("{game_pk}");
    }
}


