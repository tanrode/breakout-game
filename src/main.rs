use std::ffi::CString;

use client::screens::game;
use raylib::prelude::*;
// mod models{pub mod ball; pub mod brick; pub mod paddle;}
// mod screens{pub mod home; pub mod game; pub mod scoreboard;}
// use screens::home;
// use screens::game;
use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
mod client{pub mod helpers; pub mod models{pub mod ball; pub mod brick; pub mod paddle;} pub mod screens{pub mod home; pub mod game; pub mod scoreboard;}}
// use raylib::gui::Gui;
// use screens::game::Game;

#[derive(Serialize, Deserialize)]
struct User {
    gamer_id: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Leaderboard {
    gamer_id: String,
    high_score: i32,
    time: String,
}

#[tokio::main]
async fn main() {

    let mut is_home_screen: bool = true;
    let mut gamer_id = String::new();
    let mut password = String::new();

        while is_home_screen {
            let (input_gamer_id, input_password) = client::screens::home::get_input_from_user();
            gamer_id = input_gamer_id;
            password = input_password;
            // print!("User Details: {} -> {}", gamer_id, password);
            
            // TBD: Make API call to validate gamer_id and password, else create new account
            let result = validate_credentials(&gamer_id, &password).await;
            // println!("UNWRAP: {}", result.unwrap());
            match result {
                Ok(true) => {
                    println!("User authenticated successfully.");
                    is_home_screen = false; // Proceed to the game screen
                }
                Ok(false) => {
                    println!("Invalid credentials, please try again.");
                }
                Err(e) => {
                    println!("Error occurred during authentication: {}", e);
                }
            }

            // is_home_screen = false; 
        }

        client::screens::game:: game(&gamer_id);
}

async fn validate_credentials(gamer_id: &str, password: &str) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_url = "http://127.0.0.1:8080/user/login"; // Replace with your API endpoint

    // Prepare the payload for the POST request
    let body = serde_json::json!({
        "gamer_id": gamer_id,
        "password": password,
    });

    // Send the request
    let response = client.post(api_url)
        .json(&body)
        .send()
        .await?;

    // print!("STATUS: {}", response.status());
    // print!("{}", response.status() == StatusCode::OK);
    // print!("BODY: {}", response.text().await.unwrap());

    // Parse the response
    match response.status() {
        StatusCode::OK => {
            let api_response: Vec<User> = response.json().await.unwrap();
            // print!("{} {}", api_response[0].gamer_id, api_response[0].password);
                Ok(true) // Success case
        },
        StatusCode::UNAUTHORIZED => {
            Ok(false)
        },
        _ => {
            Err(response.error_for_status().unwrap_err())
        },
    }
}
