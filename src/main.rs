use std::ffi::CString;

use client::screens::game;
use raylib::prelude::*;
// mod screens{pub mod home; pub mod game; pub mod scoreboard;}
// use screens::home;
// use screens::game;
use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;
mod client{pub mod helpers; pub mod models{pub mod ball; pub mod brick; pub mod paddle; pub mod structures;} pub mod screens{pub mod home; pub mod game; pub mod scoreboard; pub mod start;}}
// use raylib::gui::Gui;
// use screens::game::Game;
mod api{pub mod make_calls;}

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
            let result = api::make_calls::validate_credentials(&gamer_id, &password).await;
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

        client::screens::game:: game(&gamer_id).await;
}
