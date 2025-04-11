pub fn scytale_cipher(message: String, i: u32) -> String {
    let size = i as usize;
    let message_len = message.len();

    let rows = (message_len + size - 1) / size; 

    let mut grid: Vec<Vec<char>> = vec![vec![' '; size]; rows];

    let mut idx = 0;
    for row in 0..rows {
        for col in 0..size {
            if idx < message_len {
                grid[row][col] = message.chars().nth(idx).unwrap();
                idx += 1;
            }
        }
    }

    let mut result = String::new();
    for col in 0..size {
        for row in 0..rows {
            result.push(grid[row][col]);
        }
    }

    result.trim().to_string()
}
