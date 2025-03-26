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

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard { gamer_id: "".to_string(), high_score: 0, time: "".to_string() }
    }
}