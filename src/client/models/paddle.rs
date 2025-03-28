use raylib::prelude::*;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            speed,
        }
    }
    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.x -= self.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.x += self.speed;
        }
        // Prevent going out of bounds
        self.x = self.x.clamp(0.0, 800.0 - self.width);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x as i32, self.y as i32, self.width as i32, self.height as i32, Color::WHITE);
    }
}
