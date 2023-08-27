// @generated automatically by Diesel CLI.

diesel::table! {
    leaderboard (id) {
        id -> Int4,
        name -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
    }
}
