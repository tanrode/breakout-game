use raylib::prelude::*;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Ball {
    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius, Color::WHITE);
    }
}
