pub fn num_to_ordinal(x: u32) -> String {
    let suffix = match x % 10 {
        1 if x % 100 != 11 => "st",
        2 if x % 100 != 12 => "nd",
        3 if x % 100 != 13 => "rd",
        _ => "th",
    };
    format!("{}{}", x, suffix)
}