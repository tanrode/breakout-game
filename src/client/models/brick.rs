use raylib::prelude::*;

pub struct Brick {
    pub length: i32,
    pub width: i32,
    pub color: Color,
    pub lives: i32,
    pub to_render: bool,
    pub x: f32,
    pub y: f32
}

impl Brick {
    pub fn new(color: Color, x: f32, y: f32) -> Self {
        Self {
            length: 80,
            width: 25,
            color,
            lives: 1,
            to_render: true,
            x,
            y
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x as i32, self.y as i32, self.length, self.width, self.color);
        // Draw border
        //d.draw_rectangle_lines(self.x as i32, self.y as i32, self.length, self.width, Color::WHITE);

        let border_thickness = 2;
        d.draw_rectangle_lines_ex(
            Rectangle::new(self.x, self.y, self.length as f32, self.width as f32),
            border_thickness,
            Color::GRAY,
        );

        // // Convert lives to string
        // let lives_text = self.lives.to_string();

        // // Get text dimensions to center it
        // let font_size = 20;
        // let text_width = measure_text(&lives_text, font_size);
        // let text_x = self.x as i32 + (self.length / 2) - (text_width / 2);
        // let text_y = self.y as i32 + (self.width / 2) - (font_size / 2);

        // // Draw the text
        // d.draw_text(&lives_text, text_x, text_y, font_size, Color::WHITE);
    }
}