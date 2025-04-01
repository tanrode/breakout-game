use raylib::color::Color;
use raylib::prelude::*;
use crate::client::screens::scoreboard::Scoreboard;
use crate::client::helpers;

pub struct GameOverScreen {
    render_texture: RenderTexture2D,
}

impl GameOverScreen {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let render_texture = rl.load_render_texture(thread, 800, 600).unwrap();
        Self {
            render_texture,
        }
    }

    pub fn render_static_screen(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, scoreboard: &Scoreboard, time_elapsed: &str, high_score: i32, time_taken: &str) {
            let mut d = rl.begin_texture_mode(thread, &mut self.render_texture);
            d.clear_background(Color::BLACK);
            d.draw_text(helpers::font_data::GAME_OVER_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.game_over_width) / 2, 200, helpers::font_data::FONT_SIZE, Color::WHITE);
            
            let score_text = format!("Score: {}", scoreboard.score);
            let time_text = format!("Time Taken: {} sec", time_elapsed);
            d.draw_text(&score_text, (800 - helpers::font_data::TEXT_WIDTHS.score_width) / 2, 300, helpers::font_data::FONT_SIZE_SMALL, Color::YELLOW);
            d.draw_text(&time_text, (800 - helpers::font_data::TEXT_WIDTHS.time_width) / 2, 350, helpers::font_data::FONT_SIZE_SMALL, Color::YELLOW);
            
            let best_score_text = format!("Best Score: {}", high_score);
            let best_time_text = format!("Best Time: {} sec", time_taken);
            d.draw_text(&best_score_text, (800 - helpers::font_data::TEXT_WIDTHS.best_score_width) / 2, 400, helpers::font_data::FONT_SIZE_SMALL, Color::GREEN);
            d.draw_text(&best_time_text, (800 - helpers::font_data::TEXT_WIDTHS.best_time_width) / 2, 450, helpers::font_data::FONT_SIZE_SMALL, Color::GREEN);
            
            d.draw_text(helpers::font_data::CONTINUE_TEXT, (800 - helpers::font_data::TEXT_WIDTHS.continue_width) / 2, 500, helpers::font_data::FONT_SIZE_SMALLEST, Color::GRAY);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(&self.render_texture, Rectangle::new(0.0, 0.0, 800.0, -600.0), Vector2::new(0.0, 0.0), Color::WHITE);
    }
}

pub fn handle_game_over(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    game_over_screen: &mut GameOverScreen,
) -> bool {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    game_over_screen.draw(&mut d);

    // Listen for Spacebar Press to move to the next screen
    d.is_key_pressed(KeyboardKey::KEY_SPACE)
}
