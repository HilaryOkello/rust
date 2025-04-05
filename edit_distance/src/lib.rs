pub fn edit_distance(source: &str, target: &str) -> usize {
    let n = source.len();
    let m = target.len();

    // Create a matrix to store distances between prefixes of the two strings
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Initialize the first row and column
    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }

    // Fill in the rest of the matrix
    for i in 1..=n {
        for j in 1..=m {
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

            dp[i][j] = (dp[i - 1][j] + 1) // Deletion
                .min(dp[i][j - 1] + 1) // Insertion
                .min(dp[i - 1][j - 1] + cost); // Substitution
        }
    }

    dp[n][m]
}