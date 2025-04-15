pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let numbers = s.split_whitespace();
    
    let mut result = Vec::new();
    
    for num_str in numbers {
        let mut num_str = num_str.to_string();
        let multiplier = if num_str.ends_with('k') {
            num_str.pop();
            1000
        } else {
            1
        };
        
        if let Ok(parsed_num) = num_str.parse::<f32>() {
            let final_num = (parsed_num * multiplier as f32) as u32;
            result.push(final_num);
        }
    }
    
    Box::new(result)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}