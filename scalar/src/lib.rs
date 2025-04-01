pub fn sum(a: u8, b: u8) -> u16 {
    a as u16 + b as u16
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i16 {
    a as i16 * b as i16
}

pub fn quo(a: i32, b: i32) -> i32 {
    a / b

}

pub fn rem(a: i32, b: i32) -> i32 {
    a % b
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }