pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter()
        .map(|name| {
            let parts: Vec<&str> = name.split_whitespace().collect();
            
            let mut result = String::new();
            
            for (i, part) in parts.iter().enumerate() {
                if let Some(c) = part.chars().next() {
                    result.push(c);
                    result.push('.');
                    
                    if i < parts.len() - 1 {
                        result.push(' ');
                    }
                }
            }
            
            result
        })
        .collect()
}