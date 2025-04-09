#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected_cipher = atbash_encode_str(original);

    if expected_cipher == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: expected_cipher,
        })
    } 
}

fn atbash_encode_char(c: char) -> char {
    if c.is_ascii_alphabetic() {
        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
        let end = if c.is_ascii_lowercase() { b'z' } else { b'Z' };
        (end - (c as u8 - base)) as char
    } else {
        c
    }
}

fn atbash_encode_str(original: &str) -> String {
    original.chars().map(atbash_encode_char).collect()
}