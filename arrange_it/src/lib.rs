pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(usize, String)> = Vec::new();

    for word in phrase.split_whitespace() {
        let mut number: usize = 0;
        let mut word_str = String::new();

        for c in word.chars() {
            if c.is_digit(10) {
                if let Some(n) = c.to_digit(10) {
                    number = n as usize;
                }
            } else {
                word_str.push(c);
            }
        }

        words.push((number, word_str));
    }

    words.sort_by_key(|&(num, _)| num);

    let result: String = words.into_iter().map(|(_, word)| word).collect::<Vec<String>>().join(" ");

    result
}