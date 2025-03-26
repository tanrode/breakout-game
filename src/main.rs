mod client{pub mod helpers; pub mod models{pub mod ball; pub mod brick; pub mod paddle; pub mod structures;} pub mod screens{pub mod home; pub mod game; pub mod scoreboard; pub mod start; pub mod game_play; pub mod game_over; pub mod leaderboard;}}
mod api{pub mod make_calls;}


#[tokio::main]
async fn main() {

    let mut is_home_screen: bool = true;
    let mut gamer_id = String::new();
    let mut password;

    while is_home_screen {
        let (input_gamer_id, input_password) = client::screens::home::get_input_from_user();
        gamer_id = input_gamer_id;
        password = input_password;
        
        let result = api::make_calls::validate_credentials(&gamer_id, &password).await;
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
    }

    client::screens::game:: game(&gamer_id).await;
}
