pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    
    for ch in s.chars() {
        match ch {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            },
            '+' => {
                result.push('+'); 
            },
            _ => {
                if !result.is_empty() && result[result.len() - 1] == '+' {
                    result.pop();
                } else {
                    result.push(ch);
                }
            }
        }
    }
    
    while !result.is_empty() && result[result.len() - 1] == '+' {
        result.pop();
    }
    
    *s = result.iter().collect();
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
