pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let log = if c != 0 {
        (c.abs() as f64).ln()
    } else {
        f64::NEG_INFINITY
    };
    
    (c, exp, log)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: Vec<String> = a
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .map(|n| n.exp().to_string())
        .collect();
    
    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log_values: Vec<f64> = b
        .iter()
        .map(|&n| (n.abs() as f64).ln())
        .collect();
    
    (b, log_values)
}