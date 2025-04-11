pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => {
                let a = b'a';
                (((c as u8 - a + key.rem_euclid(26) as u8) % 26) + a) as char
            }
            'A'..='Z' => {
                let a = b'A';
                (((c as u8 - a + key.rem_euclid(26) as u8) % 26) + a) as char
            }
            _ => c,
        })
        .collect()
}
