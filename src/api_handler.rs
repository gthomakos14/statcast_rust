pub async fn get_game_pks(date: String) -> Vec<String>{
    let url = format!("https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={date}");
    let response = reqwest::get(url)
        .await
        .expect("Can't query schedule API");
    let schedule_json: serde_json::Map<String, serde_json::Value> = response.json().await.unwrap();
    let dates = schedule_json.get("dates");
    // TODO: This is quite complicated. Wrap some of it in a separate function
    match dates {
        Some(x) => {
            let first_date = x.get(0);
            match first_date {
                Some(first_object) => {
                    let mut game_pks: Vec<String> = Vec::new();
                    let date_list = first_object.get("games")
                        .unwrap()
                        .as_array()
                        .unwrap();
                    for i in date_list{
                        let game_pk = i
                            .get("gamePk")
                            .unwrap()
                            .as_u64()
                            .unwrap()
                            .to_string();
                        game_pks.push(game_pk)
                    }
                    game_pks
                },
                None => Vec::new()
            }
            
        },
        None => Vec::new()
    }
}


pub async fn get_box_data(game_pk: &str){
    let url = format!("https://statsapi.mlb.com/api/v1/game/{game_pk}/boxscore");
}