use case::{Case, Casing};

fn edit_distance(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(std::cmp::min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]);
            }
        }
    }
    dp[m][n]
}

fn expected_variable(input: &str, expected: &str) -> Option<String> {
    if !input.is_snake_case() && !input.is_camel_case() {
        return None;
    }

    let input = input.to_lowercase();
    let expected = expected.to_lowercase();

    let distance = edit_distance(&input, &expected);
    let similarity = 100.0 * (1.0 - (distance as f64) / (expected.len() as f64));

    if similarity > 50.0 {
        Some(format!("{:.0}%", similarity))
    } else {
        None
    }
}