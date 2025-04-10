pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars()
        .filter(|&c| c != letter)
        .collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let lower = letter.to_ascii_lowercase();
    let upper = letter.to_ascii_uppercase();
    s.chars()
        .filter(|&c| c != lower && c != upper)
        .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let lower = letter.to_ascii_lowercase();
    let upper = letter.to_ascii_uppercase();

    s.chars()
        .map(|c| {
            if c == lower {
                c.to_ascii_uppercase()
            } else if c == upper {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}
