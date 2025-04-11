pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let text = text.to_lowercase(); 
    let chars: Vec<char> = text.chars().collect();

    if vowels.contains(&chars[0]) {
        return format!("{}ay", text);
    }

    for i in 0..chars.len() {
        if chars[i] == 'q' && i + 1 < chars.len() && chars[i + 1] == 'u' {
            return format!("{}ay", chars[i + 2..].iter().collect::<String>() + &text[0..i + 2]);
        }

        if vowels.contains(&chars[i]) {
            return format!("{}ay", chars[i..].iter().collect::<String>() + &text[0..i]);
        }
    }

    format!("{}ay", text)
}
