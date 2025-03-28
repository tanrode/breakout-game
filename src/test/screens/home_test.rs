#[cfg(test)]
mod tests {
    use crate::client::screens::home::{get_input_from_user, PasswordReader};
    use std::io::Cursor;

    // Mock implementation of PasswordReader for tests
    struct MockPasswordReader {
        mock_password: String,
    }

    impl MockPasswordReader {
        fn new(mock_password: &str) -> Self {
            MockPasswordReader {
                mock_password: mock_password.to_string(),
            }
        }
    }

    impl PasswordReader for MockPasswordReader {
        fn read_password(&mut self) -> Result<String, String> {
            Ok(self.mock_password.clone())
        }
    }

    #[test]
    fn test_valid_input() {
        let input = b"test123\nP@ssw0rd\n";
        let mut reader = Cursor::new(input);
        let password_reader = MockPasswordReader::new("P@ssw0rd");

        let credentials = get_input_from_user(&mut reader, password_reader);

        match credentials {
            Ok((gamer_id, password)) => {
                assert_eq!(gamer_id, "test123");
                assert_eq!(password, "P@ssw0rd");
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Error: GamerID must be alphanumeric and between 3 and 30 characters.")]
    fn test_invalid_gamer_id() {
        let input = b"ta\nP@ssw0rd\n";
        let mut reader = Cursor::new(input);
        let password_reader = MockPasswordReader::new("P@ssw0rd");

        let credentials = get_input_from_user(&mut reader, password_reader);

        match credentials {
            Ok((_gamer_id, _password)) => {
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Error: Password must be between 8 and 30 characters.")]
    fn test_invalid_password() {
        let input = b"test123\npass1\n"; // Invalid password (empty)
        let mut reader = Cursor::new(input);
        let password_reader = MockPasswordReader::new(""); // Empty password

        let credentials = get_input_from_user(&mut reader, password_reader);

        match credentials {
            Ok((_gamer_id, _password)) => {
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}
