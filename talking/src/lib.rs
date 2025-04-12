pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!";
    }
    let is_yelling = text.chars().any(char::is_alphabetic) && text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_question = text.ends_with('?');

    match (is_question, is_yelling) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "Sure.",
        (false, true) => "There is no need to yell, calm down!",
        (false, false) => "Interesting.",
    }
}