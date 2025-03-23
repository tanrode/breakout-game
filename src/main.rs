use std::ffi::CString;

use raylib::prelude::*;
mod models{pub mod ball; pub mod brick; pub mod paddle;}
mod screens{pub mod home; pub mod game;}
use models::ball::Ball;
use models::brick::Brick;
use models::paddle::Paddle;
use screens::home;
// use raylib::gui::Gui;
// use screens::game::Game;


// #[tokio::main]
fn main() {

    let mut is_home_screen: bool = true;
    let mut game_start: bool = false;
    let mut gamer_id = String::new();
    let mut password = String::new();

        while is_home_screen {
            let (input_gamer_id, input_password) = home::get_input_from_user();
            gamer_id = input_gamer_id;
            password = input_password;
            print!("User Details: {} -> {}", gamer_id, password);
            // TBD: Make API call to validate gamer_id and password, else create new account
            is_home_screen = false; 
        }

        let (mut rl, thread) = raylib::init().size(800, 800).title("Breakout Arcade Game").build();
        rl.set_target_fps(60);

        let mut ball = Ball {
            x: 400.0,
            y: 300.0,
            radius: 10.0,
            dx: 8.0,
            dy: 8.0,
        };

        let mut paddle = Paddle {
            x: 400.0,
            y: 700.0,
            width: 100.0,
            height: 20.0,
            speed: 16.0,
        };

        let mut bricks = Vec::new();
        let rows = 7; // Adjust as per your design
        let cols = 10; // Calculated for ideal fit
        let brick_width = 75.0;
        let brick_height = 25.0;
        // let spacing = 5.0;
        // let margin = 2.5;
        let spacing = 0.0;
        let margin = 0.0;
        let colors = vec![Color::RED, Color::ORANGE, Color::PURPLE, Color::PINK, Color::BLUE, Color::GREEN, Color::YELLOW];

        
        for row in 0..rows {
            for col in 0..cols {
                bricks.push(Brick {
                    length: brick_width as i32,
                    width: brick_height as i32,
                    color: colors[row],
                    lives: (row+1) as i32,
                    to_render: true,
                    x: margin + brick_width * (col as f32) + spacing * (col as f32),
                    y: 100.0 + brick_height * (row as f32) + spacing * (row as f32),
                });
            }
        }

        while !rl.window_should_close() {
            while !game_start {
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::RAYWHITE);

                // Draw the text field
                let welcome_message = format!("Welcome {}", gamer_id);
                d.draw_text(welcome_message.as_str(), 50, 50, 20, Color::BLACK);
                // d.draw_text(&text, 50, 80, 20, Color::BLACK);

                // Draw the button and check for click
                let button_label = CString::new("Start Game").unwrap();

                // Create the button using gui_button from raylib::gui
                if d.gui_button(Rectangle::new(350.0, 250.0, 200.0, 50.0), Some(&button_label)) {
                    game_start = true;
                }

                // d.end_drawing();
            }

            // Update Objects
            ball.update();
            paddle.update(&mut rl);
        
            // Collision Checks
            check_collision(&mut ball);
            check_collision_ball_paddle(&mut ball, &paddle);
            
            for brick in &mut bricks {
                if !brick.to_render { continue; }
                if check_collision_ball_brick(&mut ball, brick) {
                    if brick.lives == 0 {
                        brick.to_render = false;
                    }
                }
                // if circle_rect(&mut ball, brick.x,brick.y, brick.length as f32, brick.width as f32){
                //     brick.lives -= 1;
                //     if brick.lives == 0 {
                //         brick.to_render = false;
                //     }
                // }
            }
        
            // Rendering
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
        
            for brick in &bricks {
                if !brick.to_render { continue; }
                brick.draw(&mut d);
            }
            
            paddle.draw(&mut d);
            ball.draw(&mut d);
        }
}

fn check_collision(ball: &mut Ball) {
    // Wall collision
    if ball.x - ball.radius <= 0.0 || ball.x + ball.radius >= 800.0 {
        ball.dx *= -1.0;
    }
    // if ball.y - ball.radius <= 0.0 {
    //     ball.dy *= -1.0;
    // }

    // Wall collision (top and bottom - temporary till paddle & bricks logic is implemented)
    if ball.y - ball.radius <= 0.0 || ball.y + ball.radius >= 800.0 {
        ball.dy *= -1.0;
    }
}

fn check_collision_ball_paddle(ball: &mut Ball, paddle: &Paddle) {
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



// fn check_collision_ball_brick(ball: &mut Ball, brick: &mut Brick) -> bool {
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


fn check_collision_ball_brick(ball: &mut Ball, brick: &mut Brick) -> bool {
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


fn raycast_collision(ball: &mut Ball, brick: &Brick) -> bool {
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


fn circle_rect(ball: &mut Ball, rx: f32, ry: f32, rw: f32, rh: f32) -> bool {
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

