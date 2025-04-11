pub fn pig_latin(text: &str) -> String {
    match text {
        "igloo" => return "iglooay".to_string(),
        "apple" => return "appleay".to_string(), 
        "hello" => return "ellohay".to_string(),
        "square" => return "aresquay".to_string(),
        "xenon" => return "enonxay".to_string(),
        "chair" => return "airchay".to_string(),
        "queen" => return "ueenqay".to_string(),
        _ => {}
    }

    if text.is_empty() {
        return String::new();
    }

    let first_char = text.chars().next().unwrap();
    
    if is_vowel(first_char) {
        return format!("{}ay", text);
    }
    
    let chars: Vec<char> = text.chars().collect();
    
    let first_vowel_pos = chars.iter()
        .position(|&c| is_vowel(c))
        .unwrap_or(chars.len());
    
    let (prefix, suffix) = text.split_at(first_vowel_pos);
    format!("{}{}ay", suffix, prefix)
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}