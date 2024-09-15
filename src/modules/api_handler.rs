pub async fn query_url(url: String) -> serde_json::Value{
    let response = reqwest::get(url)
        .await
        .expect("Cannot query provided URL");
    let json = response
        .json()
        .await
        .expect("Poorly formed JSON. Please investigate.");
    json
}


pub async fn get_game_pks(date: String) -> Vec<String>{
    let url = format!("https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={date}");
    let schedule_json = query_url(url).await;
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


pub async fn get_box_response(game_pk: &str) -> serde_json::Value{
    let url = format!("https://statsapi.mlb.com/api/v1/game/{game_pk}/boxscore");
    let response = query_url(url).await;

    response
}


pub async fn get_pbp_response(game_pk: &str) -> serde_json::Value{
    let url = format!("https://statsapi.mlb.com/api/v1/game/{game_pk}/playByPlay");
    let response = query_url(url).await;

    response
}