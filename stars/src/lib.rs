pub fn stars(n: u32) -> String {
    let mut res = String::new();
    for _ in 0..(1 << n) {
        res.push('*');
    }
    res
}