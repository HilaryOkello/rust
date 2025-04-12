pub fn scytale_cipher(message: String, size: u32) -> String {
    let num_cols = size as usize;
    let num_rows = (message.len() as f64 / num_cols as f64).ceil() as usize;
    let mut grid: Vec<Vec<char>> = vec![vec![' '; num_cols]; num_rows];
    let mut index = 0;

    for r in 0..num_rows {
        for c in 0..num_cols {
            if index < message.len() {
                grid[r][c] = message.chars().nth(index).unwrap();
                index += 1;
            }
        }
    }

    let mut ciphertext = String::new();
    for c in 0..num_cols {
        for r in 0..num_rows {
            ciphertext.push(grid[r][c]);
        }
    }

    ciphertext.trim_end().to_string()
}

