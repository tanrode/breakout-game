use crate::client::models::{ball::Ball, brick::Brick, paddle::Paddle};

pub fn check_collision(ball: &mut Ball) -> bool {
    // Wall collision
    if ball.x - ball.radius <= 0.0 || ball.x + ball.radius >= 800.0 {
        ball.dx *= -1.0;
    }
    // if ball.y - ball.radius <= 0.0 {
    //     ball.dy *= -1.0;
    // }

    // Wall collision (top and bottom - temporary till paddle & bricks logic is implemented)
    if ball.y + ball.radius >= 800.0 {
        print!("GAME OVER");
        return false;
    }

    if ball.y - ball.radius <= 0.0 {
        ball.dy *= -1.0;
    }

    return true;
    
    
    // if ball.y - ball.radius <= 0.0 || ball.y + ball.radius >= 800.0 {
    //     ball.dy *= -1.0;
    // }

}

pub fn check_collision_ball_paddle(ball: &mut Ball, paddle: &Paddle) {
    // Check if the ball is within the paddle's horizontal range
    let ball_in_paddle_range = ball.x + ball.radius >= paddle.x && ball.x - ball.radius <= paddle.x + paddle.width;

    // Check if the ball is colliding with the paddle from above
    let ball_below_paddle = ball.y + ball.radius >= paddle.y && ball.y <= paddle.y + paddle.height && ball.dy > 0.0;

    // Check if the ball is colliding with the paddle from below (if applicable)
    let ball_above_paddle = ball.y - ball.radius <= paddle.y + paddle.height && ball.y >= paddle.y && ball.dy < 0.0;

    if ball_in_paddle_range && (ball_below_paddle || ball_above_paddle) {
        ball.dy *= -1.0;
    }
}



// pub fn check_collision_ball_brick(ball: &mut Ball, brick: &mut Brick) -> bool {
//     // Find the closest point on the rectangle to the ball
//     let closest_x: f32 = ball.x.clamp(brick.x, brick.x + brick.length as f32);
//     let closest_y: f32 = ball.y.clamp(brick.y, brick.y + brick.width as f32);

//     // Calculate the distance from the ball to this point
//     let distance_x: f32 = ball.x - closest_x;
//     let distance_y: f32 = ball.y - closest_y;
//     let distance_squared: f32 = distance_x * distance_x + distance_y * distance_y;



//     // Collision occurs if the distance is less than or equal to the ball's radius squared
//     if distance_squared <= ball.radius * ball.radius{
//         brick.lives -= 1;
//         // Simulate bounce-off effect by changing direction of ball
//         ball.dy *= -1.0;
//         return true;
//     }
//     return false;
// }


pub fn check_collision_ball_brick(ball: &mut Ball, brick: &mut Brick) -> bool {
    // Find the closest point on the rectangle to the ball for x and y
    let closest_x: f32 = ball.x.clamp(brick.x, brick.x + brick.length as f32);
    let closest_y: f32 = ball.y.clamp(brick.y, brick.y + brick.width as f32);

    // Calculate the distance from the ball to this closest point
    let distance_x: f32 = ball.x - closest_x;
    let distance_y: f32 = ball.y - closest_y;
    let distance_squared: f32 = distance_x * distance_x + distance_y * distance_y;

    // Collision occurs if the distance is less than or equal to the ball's radius squared
    if distance_squared <= ball.radius * ball.radius {
        // Simulate bounce-off effect based on which side of the brick the ball hit

        // Check for vertical collision (top/bottom)
        if closest_x == ball.x {
            ball.dy *= -1.0; // Invert vertical velocity
        }

        // Check for horizontal collision (left/right)
        if closest_y == ball.y {
            ball.dx *= -1.0; // Invert horizontal velocity
        }

        // Decrease the brick's lives
        brick.lives -= 1;

        return true;
    }

    false
}


pub fn raycast_collision(ball: &mut Ball, brick: &Brick) -> bool {
    let next_x = ball.x + ball.dx;
    let next_y = ball.y + ball.dy;

    // Check if the ray intersects with the brick's boundaries
    let intersects_x = next_x + ball.radius >= brick.x && next_x - ball.radius <= brick.x + brick.length as f32;
    let intersects_y = next_y + ball.radius >= brick.y && next_y - ball.radius <= brick.y + brick.width as f32;

    if intersects_x && intersects_y {
        // Determine collision side
        let ball_center_x = ball.x + ball.dx;
        let ball_center_y = ball.y + ball.dy;

        let brick_center_x = brick.x + brick.length as f32 / 2.0;
        let brick_center_y = brick.y + brick.width as f32 / 2.0;

        let dx = (ball_center_x - brick_center_x).abs();
        let dy = (ball_center_y - brick_center_y).abs();

        // Reflect ball based on which side it hit
        if dx > dy {
            ball.dx *= -1.0; // Horizontal reflection
        } else {
            ball.dy *= -1.0; // Vertical reflection
        }
        return true;
    }

    false
}


pub fn circle_rect(ball: &mut Ball, rx: f32, ry: f32, rw: f32, rh: f32) -> bool {
    let cx = ball.x;
    let cy = ball.y;
    let radius = ball.radius;

    // Temporary variables to set edges for testing
    let mut test_x = cx;
    let mut test_y = cy;
    

    // Determine the closest edge
    if cx < rx {
        test_x = rx; // Left edge
    } else if cx > rx + rw {
        test_x = rx + rw; // Right edge
    }

    if cy < ry {
        test_y = ry; // Top edge
    } else if cy > ry + rh {
        test_y = ry + rh; // Bottom edge
    }

    // Calculate distance from the closest edge
    let dist_x = cx - test_x;
    let dist_y = cy - test_y;
    let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();

    // Collision if distance is less than or equal to the radius
    distance <= radius
}