pub fn talking(text: &str) -> &str {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return "Just say something!";
    }
    let is_yelling = trimmed_text.chars().any(char::is_alphabetic) && trimmed_text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_question = trimmed_text.ends_with('?');

    match (is_question, is_yelling) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "Sure.",
        (false, true) => "There is no need to yell, calm down!",
        (false, false) => "Interesting.",
    }
}
