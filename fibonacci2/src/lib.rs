pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut result = 0;
            
            for _ in 2..=n {
                result = a + b;
                a = b;
                b = result;
            }
            
            result
        }
    }
}
