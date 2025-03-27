use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    gamer_id: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Leaderboard {
    pub gamer_id: String,
    pub high_score: i32,
    pub time: String,
}