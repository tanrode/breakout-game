mod client{pub mod helpers; pub mod models; pub mod screens;}
mod api{pub mod make_calls;}
mod test{pub mod api; pub mod screens;}

use std::io::{self};
use rpassword::read_password;

#[tokio::main]
async fn main() {

    let mut is_home_screen: bool = true;
    let mut gamer_id = String::new();
    let mut password;
    let base_url = "http://127.0.0.1:8080";
    let client = reqwest::Client::new();
    let mut reader = io::BufReader::new(io::stdin());
    let password_reader: fn() -> Result<String, std::io::Error> = read_password;
    // let mut password_reader = rpassword::read_password;

    while is_home_screen {
        // let (input_gamer_id, input_password) = client::screens::home::get_input_from_user(&mut reader, password_reader);
        let credentials = client::screens::home::get_input_from_user(&mut reader, password_reader);

        match credentials {
            Ok((input_gamer_id, input_password)) => {
                gamer_id = input_gamer_id;
                password = input_password;
                
                let result = api::make_calls::validate_credentials(&client, base_url, &gamer_id, &password).await;
                match result {
                    Ok(true) => {
                        println!("User authenticated successfully.");
                        is_home_screen = false; // Break out of loop
                    }
                    Ok(false) => {
                        println!("Invalid credentials, please try again.");
                    }
                    Err(e) => {
                        println!("Error occurred during authentication: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
        
        
    }

    // Proceed to the game screen
    client::screens::game:: game(&client, base_url, &gamer_id).await;
}
