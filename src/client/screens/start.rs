use raylib::prelude::*;
use std::ffi::CString;

pub fn start(gamer_id: &str, d: &mut RaylibDrawHandle) -> bool {
    // Set the background to a solid color
    d.clear_background(Color::BLACK); // Black background for contrast

    // Draw the welcome message at the center
    let welcome_message = format!("Welcome {}", gamer_id);

    // Use rl.measure_text to measure the width of the text (note rl, not d)
    let text_width = measure_text(welcome_message.as_str(), 40) as f32;
    // let text_height = 20.0;

    // Draw the welcome message centered
    d.draw_text(welcome_message.as_str(), ((800.0 - text_width) / 2.0) as i32, 150, 40, Color::WHITE);

    // Draw the start button in the center with a nice background color
    let button_label = CString::new("Start Game").unwrap();
    let button_rect = Rectangle::new(300.0, 250.0, 200.0, 50.0);

    // Button Background Color (Light Blue)
    d.draw_rectangle(button_rect.x as i32, button_rect.y as i32, button_rect.width as i32, button_rect.height as i32, Color::SKYBLUE);

    // Add a slight shadow for the text
    // d.draw_text(welcome_message.as_str(), ((800.0 - text_width) / 2.0 + 2.0) as i32, 152, 20, Color::WHITE); // Text shadow

    // Create the button using the gui_button function
    if d.gui_button(button_rect, Some(&button_label)) {
        return true;
    }

    return false;

}