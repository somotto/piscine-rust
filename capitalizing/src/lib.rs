pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => {
            let first_upper = first_char.to_uppercase().collect::<String>();
            let rest = chars.collect::<String>();
            format!("{}{}", first_upper, rest)
        }
    }
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}