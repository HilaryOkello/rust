pub fn first_subword(mut s: String) -> String {
    s = s.trim().to_string();

    if s.is_empty() {
        return String::new();
    }

    let mut first_subword = String::new();
    let mut chars = s.chars().peekable();

    if let Some(first_char) = chars.next() {
        first_subword.push(first_char);

        for c in chars {
            if c == '_' || (c.is_uppercase() && !first_subword.is_empty()) {
                break;
            }
            first_subword.push(c);
        }
    }

    first_subword
}