type X = i32;
pub fn sum(a: X, b: X) -> X {
    a + b
}

pub fn diff(a: X, b: X) -> X {
    a - b
}

pub fn pro(a: X, b: X) -> X {
    a * b
}

pub fn quo(a: X, b: X) -> X {
    if b != 0 {
        a / b
    } else {
       0
    }
}

pub fn rem(a: X, b: X) -> X {
    if b != 0 {
        a % b
    } else {
      0
    }
}
