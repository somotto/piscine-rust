pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let text = text.to_lowercase(); 
    let chars: Vec<char> = text.chars().collect();

    if vowels.contains(&chars[0]) {
        return format!("{}ay", text);
    }

    if chars.starts_with(&['q', 'u']) {
        return format!("{}ay", text[2..].to_string() + &text[0..2]);
    }

    for i in 0..chars.len() {
        if vowels.contains(&chars[i]) {
            return format!("{}ay", text[i..].to_string() + &text[0..i]);
        }
    }

    format!("{}ay", text)
}
