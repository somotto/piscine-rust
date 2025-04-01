pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        1
    } else {
        let mut result = 1;
        for i in 2..=num {
            result *= i;
        }
        result
    }
}