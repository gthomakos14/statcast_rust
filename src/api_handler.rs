use polars::prelude::*;
use std::io::Cursor;

async fn query_url(url: String) -> serde_json::Value{
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


fn format_team_data(team_data: &serde_json::Value) -> DataFrame{
    let team_name = team_data
        .get("team")
        .unwrap()
        .get("name")
        .unwrap()
        .as_str()
        .unwrap();
    let batting = team_data
        .get("teamStats")
        .unwrap()
        .get("batting")
        .unwrap()
        .to_string();
    let cursor = Cursor::new(batting.as_bytes());
    let df = JsonReader::new(cursor)
        .finish()
        .unwrap()
        .lazy()
        .with_column(lit(team_name).alias("teamName"))
        .collect()
        .unwrap();
    df
}


pub async fn get_box_data(game_pk: &str) -> DataFrame{
    let url = format!("https://statsapi.mlb.com/api/v1/game/{game_pk}/boxscore");
    let response = query_url(url).await;
    let teams_json = response.get("teams").unwrap();
    let box_data = vec![
        format_team_data(teams_json.get("away").unwrap()).lazy(),
        format_team_data(teams_json.get("home").unwrap()).lazy()];
    let lf = concat(box_data,
         UnionArgs::default()).expect("Cannot concat home and away box data")
         .with_column(lit(game_pk).alias("gamePk"));
    let df = lf.collect().unwrap();

    df
}