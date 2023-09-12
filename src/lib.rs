pub mod models;
pub mod schema;

use axum::Json;
use diesel::dsl::now;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use redis::Commands;
use redis::cluster::{ClusterClient, ClusterConnection};
use serde_json;
use std::collections::HashMap;
use std::env;
use crate::models::{NewLeaderboard, Leaderboard};
use chrono::NaiveDateTime;


const NODE1: &str = "redis://127.0.0.1:6379/";
const NODE2: &str = "redis://127.0.0.1:6378/";
const NODE3: &str = "redis://127.0.0.1:6377/";

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_redis_connection() -> ClusterConnection{
    let nodes = vec![NODE1, NODE2, NODE3];
    let client_result = ClusterClient::new(nodes);
    let client = match client_result {
        Ok(c) => c,
        Err(error) => panic!("Problem creating Redis client: {:?}", error),
    };

    let mut conn_result = client.get_connection(); 
    let mut conn = match conn_result {
        Ok(c) => c,
        Err(error) => panic!("Problem connecting to Redis: {:?}", error),
    };

    conn
}

pub fn create_new_leaderboard(name: &str, start_date: &NaiveDateTime, end_date: &NaiveDateTime) -> Leaderboard {
    use crate::schema::leaderboard;
 
    let new_leaderboard = NewLeaderboard { name, start_date, end_date };

    let conn = &mut establish_connection();
    diesel::insert_into(leaderboard::table)
        .values(&new_leaderboard)
        .returning(Leaderboard::as_returning())
        .get_result(conn)
        .expect("Error saving new leaderboard")
}


pub fn get_leaderboard_top_scores(lboard: &str, nscores: &str) -> HashMap<String, String>{
    // get the top scores in the sorted set for a given leaderboard
    // this should probably be run periodically, i.e. every minute? not sure if that will enrage players
    let leaderboard = lboard.to_string();
    let num_scores = nscores.to_string();

    let mut conn = get_redis_connection();

    // zrange board1 0 num_scores rev
    let query_result = redis::cmd("ZRANGE")
        .arg(&leaderboard)
        .arg("0")
        .arg(&num_scores)
        .arg("REV")
        .arg("WITHSCORES")
        .query::<Vec<String>>(&mut conn)
        .expect("failed to execute ZRANGE");

    let mapped_query = query_result.chunks_exact(2) // chunks_exact returns an iterator of slices
    .map(|chunk| (chunk[0].to_string(),chunk[1].to_string())) // map slices to tuples
    .collect::<HashMap<_, _>>();
    mapped_query
}


pub fn batch_update_leaderboards_top_scores() -> Json<serde_json::Value>{
    // Update the all of the leaderboard scores in Redis
    // Run periodically
    use self::schema::leaderboard::dsl::*;

    // let leaderboard = "board1";
    let num_scores = "10";

    let conn = &mut establish_connection();

    let results = leaderboard
        .filter(start_date.lt(now) )
        .filter(end_date.gt(now) )
        .select(Leaderboard::as_select())
        .load(conn)
        .expect("Error getting leaderboards");

    for r in results {
        let scores = get_leaderboard_top_scores(&r.name, num_scores);
        let result = update_leaderboard(&r.name, &scores);
    }
    let json_response = serde_json::json!({
        "status": "success",
        "message": "Finished updating leaderboards",							
    });
    Json(json_response)
}


fn update_leaderboard(leaderboard: &str, score_hashmap: &HashMap<String, String>) -> Json<serde_json::Value>{
    // update the top scores for a leaderboard
    // redis set {json of scores}

    let mapped_query_string = serde_json::to_string(score_hashmap).unwrap();
    let leaderboard_top = format!("{}_top", leaderboard.to_string());

    let mut conn = get_redis_connection();
    let _: () = conn.set(leaderboard_top, mapped_query_string).unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Updated top scores for {}", leaderboard),							
    });
    Json(json_response)
}

fn clean_redis_leaderboards() {
    // delete sorted sets after event has expired
    // Leaderboard final results are kept and should be updated before deletion
    use self::schema::leaderboard::dsl::*;

    let pgconn = &mut establish_connection();

    let results = leaderboard
        .filter(end_date.lt(now) )
        .select(Leaderboard::as_select())
        .load(pgconn)
        .expect("Error getting leaderboards");

    let mut rconn = get_redis_connection();
    let num_scores = "10";
    for r in results {
        let query_result: i32 = rconn.exists(&r.name).unwrap();
        if query_result == 1 {
            let scores = get_leaderboard_top_scores(&r.name, num_scores);
            let result = update_redis_leaderboards(&r.name, &scores);

            let del_result: i32 = rconn.del(&r.name)
            .expect(format!("Error deleting {}", &r.name).as_str());
        }
    }
}
