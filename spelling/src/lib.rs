pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n >= 1_000_000 {
        return "one million".to_string();
    }

    let ones = &[
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];
    let tens = &[
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    fn spell_under_1000(n: u64, ones: &[&str], tens: &[&str]) -> String {
        if n < 20 {
            ones[n as usize].to_string()
        } else if n < 100 {
            let ten = tens[(n / 10) as usize];
            let one = ones[(n % 10) as usize];
            if one.is_empty() {
                ten.to_string()
            } else {
                format!("{}-{}", ten, one)
            }
        } else {
            let hundred = ones[(n / 100) as usize];
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} hundred", hundred)
            } else {
                format!("{} hundred {}", hundred, spell_under_1000(remainder, ones, tens))
            }
        }
    }

    if n < 1000 {
        spell_under_1000(n, ones, tens)
    } else {
        let thousand = n / 1000;
        let remainder = n % 1000;
        let thousand_str = spell_under_1000(thousand, ones, tens);
        if remainder == 0 {
            format!("{} thousand", thousand_str)
        } else {
            let remainder_str = spell_under_1000(remainder, ones, tens);
            format!("{} thousand {}", thousand_str, remainder_str)
        }
    }
}