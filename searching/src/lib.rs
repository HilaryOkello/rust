pub fn search(array: &[i32], key: i32) -> Option<usize> {

    for (i, &item) in array.iter().enumerate() {
        if item == key {
            return Some(i as usize);
        }
    }
    None

}