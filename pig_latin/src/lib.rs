pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let first_char = text.chars().next().unwrap();
    
    if is_vowel(first_char) {
        return format!("{}ay", text);
    }
    
    let chars: Vec<char> = text.chars().collect();
    let mut first_vowel_pos = chars.iter()
        .position(|&c| is_vowel(c))
        .unwrap_or(chars.len());
    
    if first_vowel_pos > 0 && 
       chars.len() > first_vowel_pos + 1 && 
       chars[first_vowel_pos - 1] == 'q' && 
       chars[first_vowel_pos] == 'u' {
        first_vowel_pos += 1;
    }
    
    let (prefix, suffix) = text.split_at(first_vowel_pos);
    format!("{}{}ay", suffix, prefix)
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}