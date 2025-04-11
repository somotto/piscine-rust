pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = text.chars().collect();
    
    match chars.as_slice() {
        [first, ..] if is_vowel(*first) => {
            format!("{}ay", text)
        },
        
        ['q', 'u', rest @ ..] => {
            let rest_str: String = rest.iter().collect();
            format!("{}quay", rest_str)
        },
        
        _ => {
            let mut first_vowel_pos = 0;
            
            if chars.len() >= 3 {
                for i in 0..chars.len() - 2 {
                    if chars[i] != 'q' && chars[i+1] == 'q' && chars[i+2] == 'u' {
                        first_vowel_pos = i + 3;
                        break;
                    }
                }
            }
            
            if first_vowel_pos == 0 {
                first_vowel_pos = chars.iter()
                    .position(|&c| is_vowel(c))
                    .unwrap_or(chars.len());
            }
            
            let (prefix, suffix) = text.split_at(first_vowel_pos);
            format!("{}{}ay", suffix, prefix)
        }
    }
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}