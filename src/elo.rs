use std::cmp;

// Elo ranking algo taken from Beamable and translated from [scala -> ] Python -> Rust
// https://beamable.com/blog/designing-a-leaderboard-system
// Beamable algo uses Tom Kerrigan's multiplayer Elo
// http://www.tckerrigan.com/Misc/Multiplayer_Elo/

pub fn calculate_elo_expected_score(rating1: f32, rating2: f32) -> f32{
    /// for pvp
    1.0/ (1.0 + pown(10.0, ((rating2 - rating1)/400.0)))
}

pub fn calc_multiplayer_elo_delta(base: i32, loss_score: f32, win_score: f32) -> f32 {
    /// for mpvsmp
    let loss_delta = calc_delta_rating(rating_lost_to, loss_score) || 0.0;
    let win_delta = calc_delta_rating(rating_won_to, win_score) || 0.0;

    loss_delta + win_delta
}

fn elo_for_player(player_ratings: vec<f32>, player_rank_index: i32) -> (f32, f32){
    let player_rating = player_ratings[player_rank_index];

    // Assumes we "know" the rankings of all players who reported results "before" the current index
    let unknown_ratings = player_ratings[player_rank_index + 1];
    let known_ratings = player_ratings[&player_rank_index[1..len(player_rank_index)]];

    // Make the assumption that since we don't know who's going to win, let's take a minimum
    // of the players who will definitely beat us and calculate this player's elo 
    // based off of that
    let rating_lost_to = if unknown_ratings { cmp::min(unknown_ratings) } else { None };
    let rating_won_to = if unknown_ratings { cmp::max(unknown_ratings) } else { None };
    (rating_lost_to, rating_won_to)
}

fn calc_delta_rating(maybe_rating: f32, actual_score: f32) -> f32{
    if !maybe_rating.is_none() {
        expected_score = calculate_elo_expected_score(player_rating, maybe_rating);
        base * (actual_score - expected_score)
    } else {
        0.0
    }
}
