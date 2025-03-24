use raylib::prelude::*;
use std::time::Instant;

pub struct Scoreboard {
    pub score: i32,
    start_time: Instant,
}

impl Scoreboard {
    pub fn new() -> Self {
        Self {
            score: 0,
            start_time: Instant::now(),
        }
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Calculate the elapsed time
        let elapsed = self.start_time.elapsed();
        let minutes = elapsed.as_secs() / 60;
        let seconds = elapsed.as_secs() % 60;
        let time_str = format!("{:02}:{:02}", minutes, seconds);

        // Draw the Scoreboard
        d.draw_text(&format!("Score: {}", self.score), 20, 20, 30, Color::WHITE);
        d.draw_text(&format!("Time: {}", time_str), 20, 60, 30, Color::WHITE);
    }
}
