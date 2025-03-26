use raylib::prelude::*;
use lazy_static::lazy_static;

pub const GAME_OVER_TEXT: &str = "Game Over";
pub const SCORE_TEXT: &str = "Score: 100";
pub const TIME_TEXT: &str = "Time Taken: 00:00 sec";
pub const BEST_SCORE_TEXT: &str = "Best Score: 100";
pub const BEST_TIME_TEXT: &str = "Best Time: 00:00 sec";
pub const CONTINUE_TEXT: &str = "Press SPACE to Continue";
pub const HEADER_TEXT: &str = "Leaderboard";
pub const INSTRUCTION_TEXT: &str = "Press R to Restart or ESC to Exit";

pub const FONT_SIZE: i32 = 60;
pub const FONT_SIZE_SMALL: i32 = 40;
pub const FONT_SIZE_SMALLEST: i32 = 30;
pub const HEADER_FONT_SIZE: i32 = 50;
pub const INSTRUCTION_FONT_SIZE: i32 = 20;

pub struct TextWidths {
    pub game_over_width: i32,
    pub score_width: i32,
    pub time_width: i32,
    pub best_score_width: i32,
    pub best_time_width: i32,
    pub continue_width: i32,
    pub header_width: i32,
    pub instruction_width: i32
}

lazy_static! {
    pub static ref TEXT_WIDTHS: TextWidths = {
        let game_over_width = measure_text(GAME_OVER_TEXT, FONT_SIZE);
        let score_width = measure_text(SCORE_TEXT, FONT_SIZE_SMALL);
        let time_width = measure_text(TIME_TEXT, FONT_SIZE_SMALL);
        let best_score_width = measure_text(BEST_SCORE_TEXT, FONT_SIZE_SMALL);
        let best_time_width = measure_text(BEST_TIME_TEXT, FONT_SIZE_SMALL);
        let continue_width = measure_text(CONTINUE_TEXT, FONT_SIZE_SMALLEST);
        let header_width = measure_text(HEADER_TEXT, HEADER_FONT_SIZE);
        let instruction_width = measure_text(INSTRUCTION_TEXT, INSTRUCTION_FONT_SIZE);

        TextWidths {
            game_over_width,
            score_width,
            time_width,
            best_score_width,
            best_time_width,
            continue_width,
            header_width,
            instruction_width
        }
    };
}


