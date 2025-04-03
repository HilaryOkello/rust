pub fn doubtful(s: &mut String ) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubtful() {
        let mut s = "Why Rust".to_owned();
        doubtful(&mut s);
        assert_eq!(s, "Why Rust?");
    }
}
