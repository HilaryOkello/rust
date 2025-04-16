// Create the following functions:

// transform_and_save_on_heap: which accepts a string of numbers separated by spaces. If a number has a 'k' as a suffix it should be multiplied by 1000. The function transforms those numbers into a vector of u32, and saves them in the heap using Box.

// take_value_ownership: which accepts the return value from transform_and_save_on_heap, unboxes the value, and returns it.
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let nums: Vec<u32> = s
    .split_whitespace()
    .map(|s| {
        let s_trimmed = s.trim_end_matches('k');
        let mut num = match s_trimmed.parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                panic!("Warning: Could not parse '{}' as a number.", s);
            }
        };
        if s.ends_with('k') {
            num *= 1000.0;
        }
        num as u32
    })
    .collect();
    Box::new(nums)

}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}