// Instructions
// Create a function which transforms the string passed as an argument into Pig Latin:

// If a word begins with a vowel, just add "ay" to the end.
// If it begins with a consonant, then we take all consonants before the first vowel, move them to the end of the word, and then add "ay" at the end.
// If a word starts with a consonant followed by "qu", move it to the end of the word, and then add an "ay" at the end.
// Only the latin vowels will be considered as vowels (aeiou).
// Expected functions
// pub fn pig_latin(text: &str) -> String {

// }
// Usage
// Here is a program to test your function.

// use pig_latin::*;

// fn main() {
//     println!("{}", pig_latin(&String::from("igloo")));
//     println!("{}", pig_latin(&String::from("apple")));
//     println!("{}", pig_latin(&String::from("hello")));
//     println!("{}", pig_latin(&String::from("square")));
//     println!("{}", pig_latin(&String::from("xenon")));
//     println!("{}", pig_latin(&String::from("chair")));
//     println!("{}", pig_latin(&String::from("queen")));
// }
// And its output:

// $ cargo run
// iglooay
// appleay
// ellohay
// aresquay
// enonxay
// airchay
// ueenqay
// $
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
                pig_latin_word.push_str(&text[index..]);
                pig_latin_word.push_str(&text[..index]);
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