pub fn is_pangram(s: &str) -> bool {
    let mut alphabet = [false; 26];
    let mut count = 0;

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let index = c.to_ascii_lowercase() as usize - 'a' as usize;
            if !alphabet[index] {
                alphabet[index] = true;
                count += 1;
            }
        }
    }

    count == 26
}