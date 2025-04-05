pub fn sum(a: &[i32]) -> i32 {
    let mut s = 0;
    for i in a {
        s += i;
    }
    s
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}