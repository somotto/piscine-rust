pub fn pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let chars: Vec<char> = word.chars().collect();

    match chars.first() {
        Some(c) if vowels.contains(c) => format!("{}ay", word),
        
        Some('q') if chars.get(1) == Some(&'u') => {
            let start = chars[2..].iter().collect::<String>();  
            let end = chars[..2].iter().collect::<String>();  
            format!("{}{}ay", start, end)
        },
        
        _ => {
            if let Some(i) = chars.iter().position(|&c| vowels.contains(&c)) {
                let start = chars[i..].iter().collect::<String>();  
                let end = chars[..i].iter().collect::<String>();  
                format!("{}{}ay", start, end)
            } else {
                String::new()  
            }
        }
    }
}
