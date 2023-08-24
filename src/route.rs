extern crate redis;
use redis::Commands;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::handler::{
        health_checker_handler, get_player_score, get_top_scores, update_player_score
    };

/*
TODO: https://stackoverflow.com/questions/75355826/route-paths-with-or-without-of-trailing-slashes-in-rust-axum
*/
pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/get_player_score", get(get_player_score))
        .route("/api/get_top_scores", get(get_top_scores))
        .route("/api/update_player_score", post(update_player_score))
        // .with_state(app_state)
}
