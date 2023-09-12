use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::leaderboard)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Leaderboard {
    pub id: i32,
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::leaderboard)]
pub struct NewLeaderboard<'a> {
    pub name: &'a str,
    pub start_date: &'a NaiveDateTime,
    pub end_date: &'a NaiveDateTime,
}
