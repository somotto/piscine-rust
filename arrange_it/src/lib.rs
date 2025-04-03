pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(&str, usize)> = phrase
        .split_whitespace()
        .map(|word| {
            let pos = word.chars().find_map(|c| c.to_digit(10)).unwrap_or(0) as usize;
            (word, pos)
        })
        .collect();
    
    words.sort_by_key(|&(_, pos)| pos);
    
    words
        .iter()
        .map(|&(word, _)| word.chars().filter(|c| !c.is_ascii_digit()).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
