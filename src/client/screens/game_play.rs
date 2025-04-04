use raylib::prelude::*;
use crate::client::models::{ball::Ball, paddle::Paddle, brick::Brick};
use crate::client::screens::scoreboard;

pub fn game_play(d: &mut RaylibDrawHandle, ball: &mut Ball, paddle: &mut Paddle, bricks: &mut Vec<Brick>, scoreboard: &mut scoreboard::Scoreboard) {
    d.clear_background(Color::BLACK);
    scoreboard.draw(d);

    for brick in bricks {
        if !brick.to_render { continue; }
        brick.draw(d);
    }
    
    paddle.draw(d);
    ball.draw(d);
}