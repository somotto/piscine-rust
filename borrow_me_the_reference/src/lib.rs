pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            chars.remove(i);
            
            if i > 0 {
                chars.remove(i - 1);
                i -= 1; 
            }
        } else {
            i += 1;
        }
    }
    
    i = 0;
    while i < chars.len() {
        if chars[i] == '+' {
            chars.remove(i);
            
            if i < chars.len() {
                chars.remove(i);
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    
    *s = chars.iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for string in v.iter_mut() {
        let operation_idx = string.find(|c| c == '+' || c == '-').unwrap_or(0);
        
        if operation_idx > 0 && operation_idx < string.len() - 1 {
            let operation = string.chars().nth(operation_idx).unwrap();
            
            let left = string[..operation_idx].parse::<i32>().unwrap_or(0);
            let right = string[operation_idx+1..].parse::<i32>().unwrap_or(0);
            
            let result = if operation == '+' {
                left + right
            } else {
                left - right
            };
            
            *string = result.to_string();
        }
    }
}
