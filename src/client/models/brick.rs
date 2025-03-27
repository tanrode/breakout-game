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

        let border_thickness = 2;
        d.draw_rectangle_lines_ex(
            Rectangle::new(self.x, self.y, self.length as f32, self.width as f32),
            border_thickness,
            Color::GRAY,
        );
    }
}