pub fn pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let chars: Vec<char> = word.chars().collect();

    match chars.iter().position(|&c| vowels.contains(&c)) {
        Some(i) => {
            let start = chars[i..].iter().collect::<String>();
            let end = chars[..i].iter().collect::<String>();
            format!("{}{}ay", start, end)
        }
        None => String::new(),
    }
}
