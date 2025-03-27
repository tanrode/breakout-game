use crate::client::helpers;
use raylib::prelude::*;

pub fn draw_leaderboard_upper_half(d: &mut RaylibDrawHandle) {
    d.clear_background(Color::RAYWHITE);

    d.draw_text(helpers::font_data::HEADER_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.header_width) / 2, 100, helpers::font_data::HEADER_FONT_SIZE, Color::BLACK);

    let column_font_size = 30;
    let column_y = 180;
    let rank_x = 150;
    let name_x = 250;
    let score_x = 500;
    let time_x = 650;

    d.draw_text("Rank", rank_x, column_y, column_font_size, Color::DARKGRAY);
    d.draw_text("Name", name_x, column_y, column_font_size, Color::DARKGRAY);
    d.draw_text("Score", score_x, column_y, column_font_size, Color::DARKGRAY);
    d.draw_text("Time", time_x, column_y, column_font_size, Color::DARKGRAY);
}

pub fn draw_leaderboard_lower_half(d: &mut RaylibDrawHandle) -> bool {
    d.draw_text(helpers::font_data::INSTRUCTION_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.instruction_width) / 2, 700, helpers::font_data::INSTRUCTION_FONT_SIZE, Color::GRAY);

    if d.is_key_pressed(KeyboardKey::KEY_R) {
        return true;
    }

    return false;
}