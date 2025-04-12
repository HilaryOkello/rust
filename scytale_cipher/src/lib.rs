pub fn scytale_cipher(message: String, size: u32) -> String {
    let message_len = message.len();
    let num_cols = size as usize;
    let num_rows = (message_len as f32 / num_cols as f32).ceil() as usize;
    let mut grid = vec![vec![' '; num_cols]; num_rows];
    let mut index = 0;

    // Fill the grid row by row
    for r in 0..num_rows {
        for c in 0..num_cols {
            if index < message_len {
                grid[r][c] = message.chars().nth(index).unwrap();
                index += 1;
            }
        }
    }

    // Read the grid column by column
    let mut ciphertext = String::new();
    for c in 0..num_cols {
        for r in 0..num_rows {
            if r < grid.len() && c < grid[r].len() {
                ciphertext.push(grid[r][c]);
            }
        }
    }
    ciphertext
}
