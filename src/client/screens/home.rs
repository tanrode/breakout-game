use std::io::{self, Write};
use rpassword::read_password;

pub fn get_input_from_user() -> (String, String) {
    print!("Enter GamerID: ");
    io::stdout().flush().unwrap();

    let mut gamer_id = String::new();
    io::stdin().read_line(&mut gamer_id).unwrap();
    let gamer_id = gamer_id.trim().to_string();

    print!("Enter Password: ");
    io::stdout().flush().unwrap();

    let password = read_password().unwrap();
    let password = password.trim().to_string();

    (gamer_id, password)
}
