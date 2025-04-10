pub fn reverse_it(v: i32) -> String {
    let abs_str = v.abs().to_string();
    let reversed: String = abs_str.chars().rev().collect(); 
    let result = format!("{}{}", reversed, abs_str);         

    if v < 0 {
        format!("-{}", result) 
    } else {
        result
    }
}