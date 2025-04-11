pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let text = text.to_lowercase(); 
    let chars: Vec<char> = text.chars().collect();

    if vowels.contains(&chars[0]) {
        return format!("{}ay", text);
    }

    let mut first_vowel_pos = 0;
    for (i, &ch) in chars.iter().enumerate() {
        if vowels.contains(&ch) {
            first_vowel_pos = i;
            break;
        }
    }

    if first_vowel_pos > 1 && chars[0] == 'q' && chars[1] == 'u' {
        return format!("{}ay", text[2..].to_string() + &text[0..2]);
    }

    format!("{}ay", text[first_vowel_pos..].to_string() + &text[0..first_vowel_pos])
}
