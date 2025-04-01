pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i16 {
    (a as i16) * (b as i16)
}

pub fn quo(a: i32, b: i32) -> i32 {
    if b != 0 {
        a / b
    } else {
       0
    }
}

pub fn rem(a: i32, b: i32) -> i32 {
    if b != 0 {
        a % b
    } else {
      0
    }
}
