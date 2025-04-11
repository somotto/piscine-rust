pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    
    let size = (c as u8 - 'A' as u8) as usize;
    let width = 2 * size + 1;
    let mut diamond = Vec::with_capacity(width);
    
    for i in 0..=size {
        let current_char = (b'A' + i as u8) as char;
        
        if i == 0 {
            let mut row = " ".repeat(size);
            row.push('A');
            row.push_str(&" ".repeat(size));
            diamond.push(row);
        } else {
            let mut row = " ".repeat(size - i);
            row.push(current_char);
            row.push_str(&" ".repeat(2 * i - 1));
            row.push(current_char);
            row.push_str(&" ".repeat(size - i));
            diamond.push(row);
        }
    }
    
    for i in (0..size).rev() {
        let current_char = (b'A' + i as u8) as char;
        
        if i == 0 {
            let mut row = " ".repeat(size);
            row.push('A');
            row.push_str(&" ".repeat(size));
            diamond.push(row);
        } else {
            let mut row = " ".repeat(size - i);
            row.push(current_char);
            row.push_str(&" ".repeat(2 * i - 1));
            row.push(current_char);
            row.push_str(&" ".repeat(size - i));
            diamond.push(row);
        }
    }
    
    diamond
}