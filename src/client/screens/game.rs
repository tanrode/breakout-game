use raylib::color::Color;
use raylib::prelude::*;
use std::ffi::CString;
use crate::api;
use crate::client::models::structures::Leaderboard;
use crate::client::models::{ball::Ball, brick::Brick, paddle::Paddle};
use crate::client::helpers;
use crate::client::screens::scoreboard::Scoreboard;
use crate::client::screens::start;

enum GameState {
    StartScreen,
    Playing,
    GameOver,
    Leaderboard,
}

pub async fn game(gamer_id: &str) {
    let mut game_state = GameState::StartScreen;
    // let mut game_start: bool = false;

    let (mut rl, thread) = raylib::init().size(800, 800).title("Breakout Arcade Game").build();
        rl.set_target_fps(60);

        let mut first_state_visit: bool = true;

        let mut scoreboard: Scoreboard = Scoreboard::new();
        let mut time_elapsed: String = String::new();
        let mut result: Result<Vec<Leaderboard>, reqwest::Error> = Ok(Vec::new());

        // Game Objects
        let mut ball: Ball = Ball::new(300.0, 300.0, 10.0, 8.0, 8.0);
        let mut paddle: Paddle = Paddle::new(400.0, 700.0, 100.0, 20.0, 16.0);
        let mut bricks: Vec<Brick> = Vec::new();

        // Font setup
        let game_over_text = "Game Over";
        let score_text = "Score: 100";
        let time_text = "Time Taken: 00:00 sec";
        let font_size = 60;
        let font_size_small = 40;
        let text_width = measure_text(game_over_text, font_size);
        let score_width = measure_text(&score_text, font_size_small);
        let time_width = measure_text(&time_text, font_size_small);


        while !rl.window_should_close() {
            match game_state {
                GameState::StartScreen => {
                    // Initialize the starting game object states
                    ball = Ball::new(300.0, 300.0, 10.0, 8.0, 8.0);
                    paddle = Paddle::new(400.0, 700.0, 100.0, 20.0, 16.0);
                    bricks.clear();

                    // Brick wall setup parameters
                    let rows = 7;
                    let cols = 10;
                    let brick_width = 80.0;
                    let brick_height = 25.0;
                    let spacing = 0.0;
                    let margin = 0.0;
                    let colors = vec![Color::RED, Color::ORANGE, Color::PURPLE, Color::PINK, Color::BLUE, Color::GREEN, Color::YELLOW];
            
                    
                    for row in 0..rows {
                        for col in 0..cols {
                            let x = margin + brick_width * (col as f32) + spacing * (col as f32);
                            let y = 100.0 + brick_height * (row as f32) + spacing * (row as f32);
                            bricks.push(Brick::new(colors[row], x, y));
                        }
                    }

                    let mut d = rl.begin_drawing(&thread);
                    
                    if start::start(gamer_id, &mut d) {
                        first_state_visit = true;
                        scoreboard = Scoreboard::new();
                        game_state = GameState::Playing;
                    }

                    // first_state_visit = false;
                }
                GameState::Playing => {
                    // Update Objects
                    ball.update();
                    paddle.update(&mut rl);
                
                    // Collision Checks
                    let is_game_valid: bool = helpers::collisions::check_collision(&mut ball);
                    if !is_game_valid {
                        first_state_visit = true;
                        time_elapsed = scoreboard.get_time_elapsed();
                        game_state = GameState::GameOver;
                    }
                    helpers::collisions::check_collision_ball_paddle(&mut ball, &paddle);
                    
                    for brick in &mut bricks {
                        if !brick.to_render { continue; }
                        if helpers::collisions::check_collision_ball_brick(&mut ball, brick) {
                            if brick.lives == 0 {
                                brick.to_render = false;
                            }
                            scoreboard.increment_score();
                        }
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

                    first_state_visit = false;
                }
                GameState::GameOver => {
                    let mut d = rl.begin_drawing(&thread);
                    d.clear_background(Color::BLACK);

                    // Centered "Game Over" Text
                    d.draw_text(game_over_text, (800 - text_width) / 2, 200, font_size, Color::WHITE);

                    // Score and Time Display
                    let score_text = format!("Score: {}", scoreboard.score);
                    let time_text = format!("Time Taken: {} sec", time_elapsed);
                    d.draw_text(&score_text, (800 - score_width) / 2, 300, font_size_small, Color::YELLOW);
                    d.draw_text(&time_text, (800 - time_width) / 2, 350, font_size_small, Color::YELLOW);

                    // Instruction to Continue
                    let continue_text = "Press SPACE to Continue";
                    let font_size_smallest = 30;
                    let continue_width = measure_text(continue_text, font_size_smallest);
                    d.draw_text(continue_text, (800 - continue_width) / 2, 450, font_size_smallest, Color::GRAY);

                    // Check for Space Press
                    if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
                        first_state_visit = true;
                        game_state = GameState::Leaderboard;
                    }

                    // first_state_visit = false;
                }
                GameState::Leaderboard => {   
                    let mut d = rl.begin_drawing(&thread);
                    d.clear_background(Color::RAYWHITE);

                    // Header for Leaderboard
                    let header_text = "Leaderboard";
                    let header_font_size = 50;
                    let header_width = measure_text(header_text, header_font_size);
                    d.draw_text(header_text, (800 - header_width) / 2, 100, header_font_size, Color::BLACK);

                    // Column Headers
                    let column_font_size = 30;
                    let column_y = 180;
                    let rank_x = 150;
                    let name_x = 250;
                    let score_x = 500;
                    let time_x = 650;

                    d.draw_text("Rank", rank_x, column_y, column_font_size, Color::DARKGRAY);
                    d.draw_text("Name", name_x, column_y, column_font_size, Color::DARKGRAY);
                    d.draw_text("Score", score_x, column_y, column_font_size, Color::DARKGRAY);
                    d.draw_text("Time", time_x, column_y, column_font_size, Color::DARKGRAY);


                    // Fetch and Display Leaderboard Data
                    if first_state_visit {
                        result = api::make_calls::get_leaderboard().await;
                        println!("Leaderboard fetched successfully");
                        first_state_visit = false;
                    }
                    match result {
                        Ok(ref leaderboard) if !leaderboard.is_empty() => {
                            for (index, entry) in leaderboard.iter().enumerate() {
                                let rank = (index + 1).to_string();
                                let name = &entry.gamer_id;
                                let score = entry.high_score.to_string();
                                let time = &entry.time;

                                let row_y = 230 + (index as i32 * 50);

                                d.draw_text(&rank, rank_x, row_y, 25, Color::BLACK);
                                d.draw_text(name, name_x, row_y, 25, Color::BLACK);
                                d.draw_text(&score, score_x, row_y, 25, Color::BLACK);
                                d.draw_text(time, time_x, row_y, 25, Color::BLACK);
                            }
                        }
                        Ok(_) => {
                            let msg = "Leaderboard is empty. No entries found.";
                            let msg_width = measure_text(msg, 25);
                            d.draw_text(msg, (800 - msg_width) / 2, 250, 25, Color::RED);
                        }
                        Err(ref e) => {
                            let error_msg = format!("Error occurred: {}", e);
                            let error_width = measure_text(&error_msg, 25);
                            d.draw_text(&error_msg, (800 - error_width) / 2, 250, 25, Color::RED);
                        }
                    }

                    // Instructions for Restart and Exit
                    let instruction_text = "Press R to Restart or ESC to Exit";
                    let instruction_font_size = 20;
                    let instruction_width = measure_text(instruction_text, instruction_font_size);
                    d.draw_text(instruction_text, (800 - instruction_width) / 2, 700, instruction_font_size, Color::GRAY);

                    // Handle Input
                    if d.is_key_pressed(KeyboardKey::KEY_R) {
                        first_state_visit = true;
                        game_state = GameState::StartScreen;
                    }
                }
            }
        }
}