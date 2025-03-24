use raylib::color::Color;
use raylib::prelude::*;
use std::ffi::CString;
use crate::client::models::{ball::Ball, brick::Brick, paddle::Paddle};
use crate::client::helpers;
use crate::client::screens::scoreboard::Scoreboard;


pub fn game(gamer_id: &str) {
    let mut game_start: bool = false;

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
        let brick_width = 80.0;
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

        let mut scoreboard: Scoreboard = Scoreboard::new();

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
                    scoreboard = Scoreboard::new();
                }

                // d.end_drawing();
            }

            // Update Objects
            ball.update();
            paddle.update(&mut rl);
        
            // Collision Checks
            helpers::collisions::check_collision(&mut ball);
            helpers::collisions::check_collision_ball_paddle(&mut ball, &paddle);
            
            for brick in &mut bricks {
                if !brick.to_render { continue; }
                if helpers::collisions::check_collision_ball_brick(&mut ball, brick) {
                    if brick.lives == 0 {
                        brick.to_render = false;
                    }
                    scoreboard.increment_score();
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

            scoreboard.draw(&mut d);
        
            for brick in &bricks {
                if !brick.to_render { continue; }
                brick.draw(&mut d);
            }
            
            paddle.draw(&mut d);
            ball.draw(&mut d);
        }
}