use raylib::color::Color;
use raylib::prelude::*;
use reqwest::Client;
use crate::api;
use crate::client::models::structures::Leaderboard;
use crate::client::models::{ball::Ball, brick::Brick, paddle::Paddle};
use crate::client::helpers;
use crate::client::screens::scoreboard::Scoreboard;
use crate::client::screens::start;
use crate::client::screens::game_play;
use crate::client::screens::game_over;
use crate::client::screens::leaderboard;

enum GameState {
    StartScreen,
    Playing,
    GameOver,
    Leaderboard,
}

pub async fn game(client: &Client, base_url: &str, gamer_id: &str) {
    let mut game_state = GameState::StartScreen;

    let (mut rl, thread) = raylib::init().size(800, 800).title("Breakout Arcade Game").build();
        rl.set_target_fps(60);

        // Game State Flag
        let mut first_state_visit: bool = true;

        // Game Data
        let mut scoreboard: Scoreboard = Scoreboard::new();
        let mut time_elapsed: String = String::new();
        let mut result: Result<Vec<Leaderboard>, reqwest::Error> = Ok(Vec::new());
        let mut updated_stats: Result<Leaderboard, reqwest::Error>;
        let mut high_score: i32 = 0;
        let mut time_taken: String = String::new();

        // Game Objects
        let mut ball: Ball = Ball::new(300.0, 300.0, 10.0, 8.0, 8.0);
        let mut paddle: Paddle = Paddle::new(400.0, 700.0, 100.0, 20.0, 16.0);
        let mut bricks: Vec<Brick> = Vec::new();


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
                }
                GameState::Playing => {
                    // Updating Game Objects
                    ball.update();
                    paddle.update(&mut rl);
                
                    // Collision Checks
                    let is_game_valid: bool = helpers::collisions::check_collision(&mut ball);
                    if !is_game_valid || scoreboard.score == 70 {
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
                
                    // Rendering Game Objects
                    let mut d = rl.begin_drawing(&thread);
                    game_play::game_play(&mut d, &mut ball, &mut paddle, &mut bricks, &mut scoreboard);
                }
                GameState::GameOver => {
                    let mut d = rl.begin_drawing(&thread);
                    
                    // Update & fetch user's best attempt stats
                    if first_state_visit {
                        first_state_visit = false;
                        updated_stats = api::make_calls::update_leaderboard(client, base_url, gamer_id, scoreboard.score, &time_elapsed).await;
                        match updated_stats {
                            Ok(ref leaderboard) => {
                                high_score = leaderboard.high_score;
                                time_taken = leaderboard.time.clone();
                                println!("Leaderboard updated successfully");
                            }
                            Err(ref e) => {
                                println!("Error occurred while updating leaderboard: {}", e);
                            }
                        }
                    }

                    let move_to_next_screen = game_over::game_over(&mut d, &mut scoreboard, high_score, &time_taken, &time_elapsed);
                    if move_to_next_screen {
                        first_state_visit = true;
                        game_state = GameState::Leaderboard;
                    }
                }
                GameState::Leaderboard => {   
                    let mut d = rl.begin_drawing(&thread);
                    
                    // Leaderboard Data Rendering Co-ordinates
                    let rank_x = 150;
                    let name_x = 250;
                    let score_x = 500;
                    let time_x = 650;

                    leaderboard::draw_leaderboard_upper_half(&mut d);

                    // Fetch and Display Leaderboard Data
                    if first_state_visit {
                        result = api::make_calls::get_leaderboard(client, base_url).await;
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
                            let error_msg = format!("Error occurred while fetching leaderboard: {}", e);
                            print!("{}", error_msg);
                        }
                    }

                    if leaderboard::draw_leaderboard_lower_half(&mut d) {
                        first_state_visit = true;
                        game_state = GameState::StartScreen;
                    }
                }
            }
        }
}