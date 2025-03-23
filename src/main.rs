use std::ffi::CString;

use raylib::prelude::*;
mod models{pub mod ball; pub mod brick; pub mod paddle;}
mod screens{pub mod home; pub mod game; pub mod scoreboard;}
use screens::home;
use screens::game;
mod helpers;
// use raylib::gui::Gui;
// use screens::game::Game;


// #[tokio::main]
fn main() {

    let mut is_home_screen: bool = true;
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

        game:: game(&gamer_id);

        
}

