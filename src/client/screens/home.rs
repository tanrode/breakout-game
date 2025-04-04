use std::io::{self, BufRead, Write};

pub trait PasswordReader {
    fn read_password(&mut self) -> Result<String, String>;
}

impl PasswordReader for fn() -> Result<String, std::io::Error> {
    fn read_password(&mut self) -> Result<String, String> {
        match self() {
            Ok(pwd) => Ok(pwd),
            Err(_) => Err("Error reading password.".to_string()),
        }
    }
}

pub fn get_input_from_user<R: BufRead, P: PasswordReader>(mut reader: R, mut password_reader: P) -> Result<(String, String), std::io::Error> {
    let mut gamer_id = String::new();
    let mut password = String::new();

    print!("Enter GamerID: ");
    io::stdout().flush().expect("Failed to flush stdout");

    gamer_id.clear();
    if reader.read_line(&mut gamer_id).is_err() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Error Reading GamerID from Input"));
    }
    let gamer_id = gamer_id.trim().to_string();

    if gamer_id.is_empty() || gamer_id.len() < 3 || gamer_id.len() > 30 || !gamer_id.chars().all(|c| c.is_alphanumeric()) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "GamerID must be alphanumeric and between 3 and 30 characters."));
    }

    print!("Enter Password: ");
    io::stdout().flush().expect("Failed to flush stdout");

    password.clear();
    match password_reader.read_password() {
        Ok(p) => password.push_str(p.trim()),
        Err(_) => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Error reading password"));
        }
    }

    if password.is_empty() || password.len() < 8 || password.len() > 30 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Password must be between 8 and 30 characters."));
    }

    if !password.chars().any(|c| c.is_ascii_uppercase())
        || !password.chars().any(|c| c.is_ascii_digit())
        || !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Password must be at least 8 characters long and include an uppercase letter, a number, and a special character."));
    }

    Ok((gamer_id, password))
}
