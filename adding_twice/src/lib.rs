pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub fn twice<F, T>(f: F) -> impl Fn(T) -> T 
where
    F: Fn(T) -> T,
    T: Copy,
{
    move |x| f(f(x))
}