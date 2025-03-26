use raylib::prelude::*;
use crate::client::models::ball::Ball;
use crate::client::models::paddle::Paddle;
use crate::client::models::brick::Brick;
use crate::client::models::structures::Leaderboard;
use crate::client::models::structures::User;
use crate::client::screens::scoreboard;
use crate::client::helpers::collisions;

pub fn game_play(d: &mut RaylibDrawHandle, ball: &mut Ball, paddle: &mut Paddle, bricks: &mut Vec<Brick>, scoreboard: &mut scoreboard::Scoreboard) {
    // Rendering
    d.clear_background(Color::BLACK);

    scoreboard.draw(d);

    for brick in bricks {
        if !brick.to_render { continue; }
        brick.draw(d);
    }
    
    paddle.draw(d);
    ball.draw(d);
}