pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = Vec::new();
    let size = 2 * (c as u32 - 'A' as u32 + 1) - 1;

    // Top half
    for i in 0..=(c as u32 - 'A' as u32) {
        let current_char = char::from_u32((b'A' as u32) + i).unwrap();
        let leading_spaces = (size - (2 * i + 1)) / 2;
        let mut row = String::new();
        for _ in 0..leading_spaces {
            row.push(' ');
        }
        row.push(current_char);
        if current_char != 'A' {
            let inner_spaces = size - 2 * leading_spaces - 2;
            for _ in 0..inner_spaces {
                row.push(' ');
            }
            row.push(current_char);
        }
        for _ in 0..leading_spaces {
            row.push(' ');
        }
        diamond.push(row);
    }

    // Bottom half
    for i in (0..(c as u32 - 'A' as u32)).rev() {
        let current_char = char::from_u32((b'A' as u32) + i).unwrap();
        let leading_spaces = (size - (2 * i + 1)) / 2;
        let mut row = String::new();
        for _ in 0..leading_spaces {
            row.push(' ');
        }
        row.push(current_char);
        if current_char != 'A' {
            let inner_spaces = size - 2 * leading_spaces - 2;
            for _ in 0..inner_spaces {
                row.push(' ');
            }
            row.push(current_char);
        }
        for _ in 0..leading_spaces {
            row.push(' ');
        }
        diamond.push(row);
    }

    diamond
}