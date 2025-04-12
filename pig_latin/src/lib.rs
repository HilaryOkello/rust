pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    let mut pig_latin_word = String::new();
    let mut first_vowel_index = None;

    for (i, c) in text.chars().enumerate() {
        if vowels.contains(c) {
            first_vowel_index = Some(i);
            break;
        }
    }

    match first_vowel_index {
        Some(index) => {
            if index == 0 {
                pig_latin_word.push_str(text);
                pig_latin_word.push_str("ay");
            } else {
                let prefix = &text[..index];
                let suffix = &text[index..];
                if prefix.ends_with("qu") && index >= 2 {
                    pig_latin_word.push_str(suffix);
                    pig_latin_word.push_str(prefix);
                } else {
                    pig_latin_word.push_str(suffix);
                    pig_latin_word.push_str(prefix);
                }
                pig_latin_word.push_str("ay");
            }
        }
        None => {
            pig_latin_word.push_str(text);
            pig_latin_word.push_str("ay");
        }
    }

    pig_latin_word
}