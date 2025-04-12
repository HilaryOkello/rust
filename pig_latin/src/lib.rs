fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

pub fn pig_latin(text: &str) -> String {
    let word_lower = text.to_lowercase();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if let Some(first_char) = word_lower.chars().next() {
        if is_vowel(first_char) {
            return format!("{}{}", text, "ay");
        } else if word_lower.len() >= 3 && !is_vowel(word_lower.chars().nth(0).unwrap()) && word_lower.chars().nth(1) == Some('q') && word_lower.chars().nth(2) == Some('u') {
            let prefix = &text[0..3];
            let suffix = &text[3..];
            return format!("{}{}{}", suffix, prefix, "ay");
        } else {
            let mut first_vowel_index = None;
            for (i, c) in word_lower.chars().enumerate() {
                if vowels.contains(&c) {
                    first_vowel_index = Some(i);
                    break;
                }
            }

            if let Some(index) = first_vowel_index {
                let prefix = &text[0..index];
                let suffix = &text[index..];
                return format!("{}{}{}", suffix, prefix, "ay");
            } else {
                return format!("{}{}", text, "ay");
            }
        }
    } else {
        return String::new();
    }
}