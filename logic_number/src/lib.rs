pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len();
    let mut n = num
    let mut sum = 0;

    while n > 0 {
        let digit = n % 10;
        sum += digit.pow(len as u32);
        n /= 10;
    }
    sum == num
}