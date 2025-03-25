use raylib::prelude::*;
use std::ffi::CString;

pub fn start(gamer_id: &str, d: &mut RaylibDrawHandle) -> bool {
    d.clear_background(Color::BLACK);

    // Draw the text field
    let welcome_message = format!("Welcome {}", gamer_id);
    d.draw_text(welcome_message.as_str(), 50, 50, 20, Color::WHITE);

    // Draw the button and check for click
    let button_label = CString::new("Start Game").unwrap();

    // Create the button using gui_button from raylib::gui
    if d.gui_button(Rectangle::new(350.0, 250.0, 200.0, 50.0), Some(&button_label)) {
        return true;
    }   

    return false;
}