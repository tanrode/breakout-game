use raylib::color::Color;
use raylib::prelude::*;
use crate::client::screens::scoreboard::Scoreboard;
use crate::client::helpers;

pub fn game_over(d: &mut RaylibDrawHandle, scoreboard: &mut Scoreboard, high_score: i32, time_taken: &str, time_elapsed: &str) -> bool {
    d.clear_background(Color::BLACK);

    // Centered "Game Over" Text
    d.draw_text(helpers::font_data::GAME_OVER_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.game_over_width) / 2, 200, helpers::font_data::FONT_SIZE, Color::WHITE);

    // Current Session's Score & Time
    let score_text = format!("Score: {}", scoreboard.score);
    let time_text = format!("Time Taken: {} sec", time_elapsed);
    d.draw_text(&score_text, (800 - helpers::font_data::TEXT_WIDTHS.score_width) / 2, 300, helpers::font_data::FONT_SIZE_SMALL, Color::YELLOW);
    d.draw_text(&time_text, (800 - helpers::font_data::TEXT_WIDTHS.time_width) / 2, 350, helpers::font_data::FONT_SIZE_SMALL, Color::YELLOW);

    // User's Best Score & Time
    let best_score_text = format!("Best Score: {}", high_score);
    let best_time_text = format!("Best Time: {} sec", time_taken);
    d.draw_text(&best_score_text, (800 - helpers::font_data::TEXT_WIDTHS.best_score_width) / 2, 400, helpers::font_data::FONT_SIZE_SMALL, Color::GREEN);
    d.draw_text(&best_time_text, (800 - helpers::font_data::TEXT_WIDTHS.best_time_width) / 2, 450, helpers::font_data::FONT_SIZE_SMALL, Color::GREEN);

    d.draw_text(helpers::font_data::CONTINUE_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.continue_width) / 2, 500, helpers::font_data::FONT_SIZE_SMALLEST, Color::GRAY);

    // Check for Spacebar Press
    if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
        return true;
    }

    return false;
}