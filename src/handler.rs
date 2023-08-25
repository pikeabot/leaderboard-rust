use std::sync::Arc;
use std::collections::HashMap;
use redis::{ RedisError, FromRedisValue, RedisResult};
use redis::Commands;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "I am healthy";

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

    let leaderboard: String = params.get("leaderboard").unwrap().to_string();
    let player: String = params.get("player").unwrap().to_string();

    let client_result = redis::Client::open("redis://127.0.0.1:6379/");
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
        .expect("failed to execute ZRANGE");


    let json_response = serde_json::json!({
        "status": "success",
        "message": query_result[0],
    });
    Json(json_response)
}


pub async fn update_player_score() -> impl IntoResponse {
    const MESSAGE: &str = "Get Score";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}


pub async fn get_top_scores() -> impl IntoResponse {
    const MESSAGE: &str = "Get Score";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}



pub async fn create_leaderboard() -> impl IntoResponse {
    const MESSAGE: &str = "Get Score";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
