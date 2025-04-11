pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = Vec::new();
    
    let n = (c as u8 - 'A' as u8) as usize;
    
    for i in 0..=n {
        let letter = (b'A' + i as u8) as char;
        let spaces = n - i;
        if i == 0 {
            diamond.push(format!("{:width$}", letter, width = n));
        } else {
            diamond.push(format!("{:width$}{}{:width$}", letter, " ".repeat(2 * i - 1), letter, width = spaces));
        }
    }

    for i in (0..n).rev() {
        diamond.push(diamond[i].clone());
    }

    diamond
}
