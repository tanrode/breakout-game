use raylib::color::Color;
use reqwest::Client;
use crate::api;
use crate::client::models::{ball::Ball, brick::Brick, paddle::Paddle, structures::Leaderboard};
use crate::client::helpers;
use crate::client::screens::{start::{StartScreen, handle_game_start}, game_play::game_play, leaderboard::{LeaderboardScreen, handle_leaderboard_rendering}, game_over::{GameOverScreen, handle_game_over}, scoreboard::Scoreboard};

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
        let mut _result: Result<Vec<Leaderboard>, reqwest::Error> = Ok(Vec::new());
        let mut updated_stats: Result<Leaderboard, reqwest::Error>;

        // Game Objects
        let mut ball: Ball = Ball::new(300.0, 300.0, 10.0, 8.0, 8.0);
        let mut paddle: Paddle = Paddle::new(400.0, 700.0, 100.0, 20.0, 16.0);
        let mut bricks: Vec<Brick> = Vec::new();

        // Game Screens
        let mut game_start_screen = StartScreen::new(&mut rl, &thread);
        let mut game_over_screen = GameOverScreen::new(&mut rl, &thread);
        let mut leaderboard_screen = LeaderboardScreen::new(&mut rl, &thread);


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

                    if first_state_visit {
                        game_start_screen.render_static_screen(&mut rl, &thread, gamer_id);
                        first_state_visit = false;
                    }
                    else{
                        if handle_game_start(&mut rl, &thread, &mut game_start_screen) {
                            first_state_visit = true;
                            scoreboard = Scoreboard::new();
                            game_state = GameState::Playing;
                        }
                    }

                }
                GameState::Playing => {
                    // Updating Game Objects
                    ball.update();
                    paddle.update(&mut rl);
                
                    // Collision Checks
                    let is_game_valid: bool = helpers::collisions::check_collision_ball_wall(&mut ball);
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
                    game_play(&mut d, &mut ball, &mut paddle, &mut bricks, &mut scoreboard);
                }
                GameState::GameOver => {
                    // Update & fetch user's best attempt stats
                    if first_state_visit {
                        first_state_visit = false;
                        updated_stats = api::make_calls::update_leaderboard(client, base_url, gamer_id, scoreboard.score, &time_elapsed).await;
                        match updated_stats {
                            Ok(ref leaderboard) => {
                                println!("Leaderboard updated successfully");
                                game_over_screen.render_static_screen(&mut rl, &thread, &scoreboard, &time_elapsed, leaderboard.high_score, &leaderboard.time.clone());
                            }
                            Err(ref e) => {
                                println!("Error occurred while updating leaderboard: {}", e);
                            }
                        }
                    }
                
                    let move_to_next_screen = handle_game_over(&mut rl, &thread, &mut game_over_screen);
                    if move_to_next_screen {
                        first_state_visit = true;
                        game_state = GameState::Leaderboard;
                    }
                }
                GameState::Leaderboard => {   

                    // Fetch Leaderboard Data
                    if first_state_visit {
                        _result = api::make_calls::get_leaderboard(client, base_url).await;
                        println!("Leaderboard fetched successfully");
                        first_state_visit = false;
                    
                        match _result {
                            Ok(ref leaderboard) if !leaderboard.is_empty() => {
                                let mut leaderboard_entries: Vec<Leaderboard> = Vec::new();
                                for entry in leaderboard {
                                    leaderboard_entries.push(Leaderboard {
                                        gamer_id: entry.gamer_id.clone(),
                                        high_score: entry.high_score,
                                        time: entry.time.clone(),
                                    });
                                }
                                leaderboard_screen.render_static_screen(&mut rl, &thread, &leaderboard_entries);
                            }
                            Ok(_) => {
                                leaderboard_screen.render_for_empty_leaderboard(&mut rl, &thread);
                            }
                            Err(ref e) => {
                                let error_msg = format!("Error occurred while fetching leaderboard: {}", e);
                                print!("{}", error_msg);
                            }
                        }
                    }

                    if handle_leaderboard_rendering(&mut rl, &thread, &mut leaderboard_screen) {
                        first_state_visit = true;
                        game_state = GameState::StartScreen;
                    }
                }
            }
        }
}