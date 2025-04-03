pub fn str_len(s: &str) -> usize {
    s.chars().count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_len() {
        assert_eq!(str_len("Hello"), 5);
    }
}
