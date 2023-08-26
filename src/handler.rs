use std::sync::Arc;
use std::collections::HashMap;
use redis::{ RedisError, FromRedisValue, RedisResult};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
// use serde_json::json;
use serde::Deserialize;

const REDIS_HOST: &str = "redis://127.0.0.1:6379/";


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "I am healthy";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn create_leaderboard() -> impl IntoResponse {
    const MESSAGE: &str = "create_leaderboard";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}


/*
TODO: Single container for now. Probably want to use a cluster in the future
 */
pub async fn get_player_score(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // Get the player score from redis

    let leaderboard: String = params.get("leaderboard").unwrap().to_string();
    let player: String = params.get("player").unwrap().to_string();

    let client_result = redis::Client::open(REDIS_HOST);
    let client = match client_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    let mut conn_result = client.get_connection(); 
    let mut conn = match conn_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    let query_result: Vec<String> = redis::cmd("ZSCORE")
        .arg(leaderboard)
        .arg(player)
        .query::<Vec<String>>(&mut conn)
        .expect("failed to execute ZSCORE");


    let json_response = serde_json::json!({
        "status": "success",
        "message": query_result[0],
    });
    Json(json_response)
}


#[derive(Deserialize)]
pub struct PlayerScore {
    leaderboard: String,
    player: String,
    score: String,
}

pub async fn update_player_score(Json(payload): Json<PlayerScore>,) -> (StatusCode, Json<serde_json::Value>) {
// pub async fn update_player_score(Query(axum::extract::Json(payload)): Query<axum::extract::Json<PlayerScore>>) -> impl IntoResponse {
//     // pub async fn update_player_score() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let leaderboard: String = payload.leaderboard;
    let score: String = payload.score;
    let player: String = payload.player;

    // let leaderboard: String = String::from("board1");
    // let score: String = String::from("5");
    // let player: String = String::from("player1");

    let client_result = redis::Client::open(REDIS_HOST);
    let client = match client_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    let mut conn_result = client.get_connection(); 
    let mut conn = match conn_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    let query_result: i32= redis::cmd("ZADD")
        .arg(&leaderboard)
        .arg(&score)
        .arg(&player)
        .query(&mut conn)
        .expect("failed to execute ZADD");


    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Updated score to {} for {}", score, player),							
    });
    (StatusCode::OK, Json(json_response))
}

pub async fn get_top_scores(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // Get top player scores
   
    let leaderboard: String = params.get("leaderboard").unwrap().to_string();
    let num_scores: String = params.get("num_scores").unwrap().to_string();

    let client_result = redis::Client::open(REDIS_HOST);
    let client = match client_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    let mut conn_result = client.get_connection(); 
    let mut conn = match conn_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    // zrange board1 0 num_scores rev
    redis::cmd("ZRANGE")
        .arg(leaderboard)
        .arg("0")
        .arg(num_scores)
        .arg("REV")
        .arg("WITHSCORES")
        .query::<Vec<String>>(&mut conn)
        .expect("failed to execute ZRANGE");

    let mut scores = json::JsonValue::new_object();

    // let n = num_scores.parse::<i32>().unwrap();
    // for i in (0..n-1).step_by(2) {
    //     scores[query_result[i]] = query_result[i+1].into();
    // }   

    let json_response = serde_json::json!({
        "status": "success",
        "message": "score updated",
    });
    Json(json_response)
}
