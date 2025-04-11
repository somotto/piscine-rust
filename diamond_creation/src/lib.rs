pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = Vec::new();
    
    let n = (c as u8 - 'A' as u8) as usize;
    
    let width = 2 * n + 1;

    for i in 0..=n {
        let letter = (b'A' + i as u8) as char;
        let spaces = n - i;
        
        let row = match i {
            0 => format!("{:width$}", letter),  
            _ => format!("{:width$}{}{:width$}", letter, " ".repeat(2 * i - 1), letter, width = spaces),
        };
        diamond.push(row);
    }

    for i in (0..n).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}
