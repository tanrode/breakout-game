use std::io::{self, Write};
use rpassword::read_password;

pub fn get_input_from_user() -> (String, String) {
    // Prompt for GamerID
    print!("Enter GamerID: ");
    io::stdout().flush().unwrap(); // Ensures the prompt is displayed immediately

    let mut gamer_id = String::new();
    io::stdin().read_line(&mut gamer_id).unwrap();
    let gamer_id = gamer_id.trim().to_string(); // Remove leading/trailing whitespace

    // Prompt for Password
    print!("Enter Password: ");
    io::stdout().flush().unwrap(); // Ensures the prompt is displayed immediately

    let password = read_password().unwrap();
    let password = password.trim().to_string(); // Remove leading/trailing whitespace

    (gamer_id, password) // Return both gamer_id and password as a tuple
}
