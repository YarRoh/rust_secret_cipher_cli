pub fn encrypt(message: &str, shift: u8) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = ((c as u8 - base + shift) % 26) + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decrypt(message: &str, shift: u8) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shifted = ((c as u8 - base + 26 - shift) % 26) + base;
                shifted as char
            } else {
                c
            }
        })
        .collect()
}

pub fn brute_force_decrypt(message: &str) -> Vec<String> {
    (1..=26)
        .map(|shift| decrypt(message, shift))
        .collect()
}