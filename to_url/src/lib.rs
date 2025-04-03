pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        assert_eq!(test_to_url("Hello, World!"), "Hello,%20world!");
    }
}
