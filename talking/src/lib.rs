pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        return "Just say something!";
    }
    match text.ends_with('?') {
        true => match text.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            true => return "Quiet, I am thinking!",
            false => return "Sure.",
        },
        false => match text.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            true => return "There is no need to yell, calm down!",
            false => return "Interesting.",
        },
    }
}