pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len();
    let mut num = num;
    let mut sum = 0;

    while num > 0 {
        let digit = num % 10;
        sum += digit.pow(len as u32);
        num /= 10;
    }
    sum == num
}