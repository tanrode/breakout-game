use crate::client::models::{ball::Ball, brick::Brick, paddle::Paddle};

pub fn check_collision(ball: &mut Ball) -> bool {
    // Wall collision (Left & Right Walls)
    if ball.x - ball.radius <= 0.0 || ball.x + ball.radius >= 800.0 {
        ball.dx *= -1.0;
    }

    // Wall collision (top of the window)
    if ball.y - ball.radius <= 0.0 {
        ball.dy *= -1.0;
    }

    // Wall collision (bottom of the window - Game Over)
    if ball.y + ball.radius >= 800.0 {
        print!("GAME OVER");
        return false;
    }

    return true;
}

pub fn check_collision_ball_paddle(ball: &mut Ball, paddle: &Paddle) {
    // Check if the ball is within the paddle's horizontal range
    let ball_in_paddle_range = ball.x + ball.radius >= paddle.x && ball.x - ball.radius <= paddle.x + paddle.width;

    // Check if the ball is colliding with the paddle from above
    let ball_below_paddle = ball.y + ball.radius >= paddle.y && ball.y <= paddle.y + paddle.height && ball.dy > 0.0;

    // Check if the ball is colliding with the paddle from below (Safety check & normally won't occur)
    let ball_above_paddle = ball.y - ball.radius <= paddle.y + paddle.height && ball.y >= paddle.y && ball.dy < 0.0;

    // Collision occurs
    if ball_in_paddle_range && (ball_below_paddle || ball_above_paddle) {
        ball.dy *= -1.0;
    }
}


pub fn check_collision_ball_brick(ball: &mut Ball, brick: &mut Brick) -> bool {
    // Closest point on the brick to the ball
    let closest_x: f32 = ball.x.clamp(brick.x, brick.x + brick.length as f32);
    let closest_y: f32 = ball.y.clamp(brick.y, brick.y + brick.width as f32);

    // Distance from the ball to this closest point
    let distance_x: f32 = ball.x - closest_x;
    let distance_y: f32 = ball.y - closest_y;
    let distance_squared: f32 = distance_x * distance_x + distance_y * distance_y;

    // Collision occurs
    if distance_squared <= ball.radius * ball.radius {
        // Check for vertical collision (top/bottom)
        if closest_x == ball.x {
            ball.dy *= -1.0;
        }

        // Check for horizontal collision (left/right)
        if closest_y == ball.y {
            ball.dx *= -1.0;
        }

        // Decrease the brick's lives
        brick.lives -= 1;

        return true;
    }

    false
}