pub fn sum(a: &[i32]) -> i32 {
    let mut total = 0;
    for i in a {
        total += i;
    }
    total
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}