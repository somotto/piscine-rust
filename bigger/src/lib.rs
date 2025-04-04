use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values()         
        .filter(|&x| *x > 0)  
        .max()         
        .unwrap_or(&0) 
        .clone()       
}
