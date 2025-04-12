pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    let key = key.rem_euclid(26); // Ensure the key is within 0-25 range

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let rotated = ((c as u8 - base + key as u8) % 26) + base;
            result.push(rotated as char);
        } else {
            result.push(c);
        }
    }

    result
}