use polars::prelude::*;
use std::io::Cursor;
use super::api_handler;


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
    let response = api_handler::get_box_response(game_pk).await;
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


pub async fn get_play_data(game_pk: &str){
    let url = format!("https://statsapi.mlb.com/api/v1/game/{game_pk}/playByPlay");
    let response = api_handler::query_url(url).await;

    let all_plays = response.get("allPlays")
        .unwrap()
        .as_array()
        .unwrap();
    for play in all_plays{
        let event_type = play.get("result")
            .unwrap()
            .get("eventType")
            .unwrap()
            .as_str()
            .unwrap();
        let matchup = play.get("matchup").unwrap();
        let bat_side = matchup.get("batSide")
            .unwrap()
            .get("code")
            .unwrap()
            .as_str()
            .unwrap();
        let batter = play.get("batter").unwrap();
        let batter_id = batter.get("id")
            .unwrap()
            .as_str()
            .unwrap();
        let batter_name = batter.get("fullName")
            .unwrap()
            .as_str()
            .unwrap();
    }
}