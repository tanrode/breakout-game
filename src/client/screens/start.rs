use raylib::prelude::*;
use std::ffi::CString;

pub struct StartScreen {
    render_texture: RenderTexture2D,
    button_rect: Rectangle,
    button_label: CString,
}

impl StartScreen {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let render_texture = rl.load_render_texture(thread, 800, 600).unwrap();
        Self {
            render_texture,
            button_rect: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            button_label: CString::new("Start Game").unwrap(),
        }
    }

    pub fn render_static_screen(&mut self, mut rl: &mut RaylibHandle, thread: &RaylibThread, gamer_id: &str) {
        let mut d = rl.begin_texture_mode(thread, &mut self.render_texture);

        d.clear_background(Color::BLACK);

        let welcome_message = format!("Welcome {}", gamer_id);
        let text_width = measure_text(welcome_message.as_str(), 40) as f32;
        d.draw_text(welcome_message.as_str(), ((800.0 - text_width) / 2.0) as i32, 150, 40, Color::WHITE);

        self.button_rect = Rectangle::new(300.0, 250.0, 200.0, 50.0);
        d.draw_rectangle(self.button_rect.x as i32, self.button_rect.y as i32, self.button_rect.width as i32, self.button_rect.height as i32, Color::SKYBLUE);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(&self.render_texture, Rectangle::new(0.0, 0.0, 800.0, -600.0), Vector2::new(0.0, 0.0), Color::WHITE);
    }
}

pub fn handle_game_start(rl: &mut RaylibHandle, thread: &RaylibThread, start_screen: &mut StartScreen) -> bool {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::BLACK);
    start_screen.draw(&mut d);
    
    if d.gui_button(start_screen.button_rect, Some(&start_screen.button_label)) {
        return true;
    }

    return false;
}

