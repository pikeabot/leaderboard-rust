pub mod models;
pub mod schema;

use diesel::{pg::PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewLeaderboard, Leaderboard};
use chrono::NaiveDateTime;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



pub fn create_new_leaderboard(conn: &mut PgConnection, name: &str, start_date: &NaiveDateTime, end_date: &NaiveDateTime) -> Leaderboard {
    use crate::schema::leaderboard;
 
    let new_leaderboard = NewLeaderboard { name, start_date, end_date };

    diesel::insert_into(leaderboard::table)
        .values(&new_leaderboard)
        .returning(Leaderboard::as_returning())
        .get_result(conn)
        .expect("Error saving new leaderboard")
}
