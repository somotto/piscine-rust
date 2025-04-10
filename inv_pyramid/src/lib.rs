pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();

    // Create the top half of the pyramid
    for j in 1..=i {
        let spaces = " ".repeat(j as usize);
        let symbols = v.repeat(j as usize);
        result.push(format!("{}{}", spaces, symbols));
    }

    // Create the bottom half of the pyramid
    for j in (1..i).rev() {
        let spaces = " ".repeat(j as usize);
        let symbols = v.repeat(j as usize);
        result.push(format!("{}{}", spaces, symbols));
    }

    result
}
