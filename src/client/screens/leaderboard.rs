use crate::client::{helpers, models::structures::Leaderboard};
use raylib::prelude::*;

pub struct LeaderboardScreen {
    render_texture: RenderTexture2D,
    rank_x: i32,
    name_x: i32,
    score_x: i32,
    time_x: i32,
    y: i32,
    column_font_size: i32
}

impl LeaderboardScreen {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> LeaderboardScreen {
        let render_texture = rl.load_render_texture(thread, 800, 600).unwrap();
        Self {
            render_texture,
            rank_x: 150,
            name_x: 250,
            score_x: 500,
            time_x: 650,
            y: 180,
            column_font_size: 30
        }
    }

    pub fn render_static_screen(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, leaderboard: &Vec<Leaderboard>) {
        let mut d = rl.begin_texture_mode(thread, &mut self.render_texture);
        
        // Draw Leaderboard Header
        d.clear_background(Color::RAYWHITE);

        d.draw_text(helpers::font_data::HEADER_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.header_width) / 2, 100, helpers::font_data::HEADER_FONT_SIZE, Color::BLACK);

        for i in 0..=leaderboard.len() {
            let index = i as i32;
            let y_pos = self.y + (index as i32 * 50);
            if index == 0 {
                d.draw_text("Rank", self.rank_x, y_pos, self.column_font_size, Color::DARKGRAY);
                d.draw_text("Name", self.name_x, y_pos, self.column_font_size, Color::DARKGRAY);
                d.draw_text("Score", self.score_x, y_pos, self.column_font_size, Color::DARKGRAY);
                d.draw_text("Time", self.time_x, y_pos, self.column_font_size, Color::DARKGRAY);
            }
            else{
                d.draw_text(&index.to_string(), self.rank_x, y_pos, self.column_font_size, Color::DARKGRAY);
                d.draw_text(&leaderboard[(index-1) as usize].gamer_id, self.name_x, y_pos, self.column_font_size, Color::DARKGRAY);
                d.draw_text(&leaderboard[(index-1) as usize].high_score.to_string(), self.score_x, y_pos, self.column_font_size, Color::DARKGRAY);
                d.draw_text(&leaderboard[(index-1) as usize].time, self.time_x, y_pos, self.column_font_size, Color::DARKGRAY);
            }
        }
    }

    pub fn render_for_empty_leaderboard(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_texture_mode(thread, &mut self.render_texture);
        let msg = "Leaderboard is empty. No entries found.";
        let msg_width = measure_text(msg, 25);
        d.draw_text(msg, (800 - msg_width) / 2, 250, 25, Color::RED);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(&self.render_texture, Rectangle::new(0.0, 0.0, 800.0, -600.0), Vector2::new(0.0, 0.0), Color::WHITE);
    }
}

pub fn handle_leaderboard_rendering(rl: &mut RaylibHandle, thread: &RaylibThread, leaderboard_screen: &mut LeaderboardScreen) -> bool {
    let mut d = rl.begin_drawing(thread);
    
    d.clear_background(Color::RAYWHITE);
    leaderboard_screen.draw(&mut d);

    // Instruction Text
    d.draw_text(helpers::font_data::INSTRUCTION_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.instruction_width) / 2, 700, helpers::font_data::INSTRUCTION_FONT_SIZE, Color::GRAY);
    
    // Listen for R/ESC key press
    if d.is_key_pressed(KeyboardKey::KEY_R) {
        return true;
    }

    return false;
}