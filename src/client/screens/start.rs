use raylib::prelude::*;
use std::ffi::CString;

pub fn start(gamer_id: &str, d: &mut RaylibDrawHandle) -> bool {
    d.clear_background(Color::BLACK);

    let welcome_message = format!("Welcome {}", gamer_id);
    let text_width = measure_text(welcome_message.as_str(), 40) as f32;
    d.draw_text(welcome_message.as_str(), ((800.0 - text_width) / 2.0) as i32, 150, 40, Color::WHITE);

    let button_label = CString::new("Start Game").unwrap();
    let button_rect = Rectangle::new(300.0, 250.0, 200.0, 50.0);
    d.draw_rectangle(button_rect.x as i32, button_rect.y as i32, button_rect.width as i32, button_rect.height as i32, Color::SKYBLUE);

    if d.gui_button(button_rect, Some(&button_label)) {
        return true;
    }

    return false;
}