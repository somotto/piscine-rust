pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut skip_next = false;
    
    for i in 0..chars.len() {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match chars[i] {
            '-' => {
                result.pop();
            }
            '+' => {
                skip_next = true;
            }
            c => {
                result.push(c);
            }
        }
    }
    
    *s = result.into_iter().collect();
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
