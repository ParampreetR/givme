use rand::random;

pub const BANNER: &str =
    "████████████████████████████████████████████████████████████████████████████████
█░░░░░░░░░░░░░░█░░░░░░░░░░█░░░░░░██░░░░░░█░░░░░░██████████░░░░░░█░░░░░░░░░░░░░░█
█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀░░█░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░░░░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░▄▀░░░░░░░░░░█░░░░▄▀░░░░█░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░███████████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░▄▀░░░░░░▄▀░░█░░▄▀░░█████████
█░░▄▀░░███████████░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░██░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░██░░░░░░███░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░▄▀░░██░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░▄▀░░██░░▄▀░░███░░▄▀░░███░░▄▀░░██░░▄▀░░█░░▄▀░░██░░░░░░██░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀░░██░░▄▀░░███░░▄▀░░███░░▄▀▄▀░░▄▀▄▀░░█░░▄▀░░██████████░░▄▀░░█░░▄▀░░█████████
█░░▄▀░░░░░░▄▀░░█░░░░▄▀░░░░█░░░░▄▀▄▀▄▀░░░░█░░▄▀░░██████████░░▄▀░░█░░▄▀░░░░░░░░░░█
█░░▄▀▄▀▄▀▄▀▄▀░░█░░▄▀▄▀▄▀░░███░░░░▄▀░░░░███░░▄▀░░██████████░░▄▀░░█░░▄▀▄▀▄▀▄▀▄▀░░█
█░░░░░░░░░░░░░░█░░░░░░░░░░█████░░░░░░█████░░░░░░██████████░░░░░░█░░░░░░░░░░░░░░█
████████████████████████████████████████████████████████████████████████████████";

/// Returns random sequence of characters with provided length
///
/// Bydefault it avoids any "'" in randomly generated String.
/// This is done to avoid any errors while giving this String
/// in SQL Command.
pub fn rand_string(size: usize) -> String {
    loop {
        let random_string: String = (0..size)
            .map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char)
            .collect();
        if !random_string.contains("'") {
            return random_string;
        }
    }
}

/// As a security measure and shortcommings of encryption algorithms.
/// We need to adjust password to some specified length. This is done
/// by repeating password couple of times.
pub fn adjust_password_length(input_password: &str, length: usize) -> String {
    let mut proper_length_password = String::new();
    if input_password.len() > length {
        eprintln!("Slicing password to first length digits...");
        proper_length_password = input_password[..length].to_string();
    } else {
        loop {
            for single_char in input_password.chars() {
                proper_length_password.push(single_char);
                if proper_length_password.len() == length {
                    break;
                }
            }
            if proper_length_password.len() == length {
                break;
            }
        }
    }
    proper_length_password
}
