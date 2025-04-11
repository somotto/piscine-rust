pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_yelling = text.chars().any(char::is_alphabetic) && 
                     !text.chars().any(char::is_lowercase);
    
    let is_question = text.trim_end().ends_with('?');

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}