pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

use std::ops::Add;
use std::cmp::PartialOrd;
use std::marker::Copy;

impl<T> StepIterator<T> 
where
    T: Add<Output = T> + PartialOrd + Copy + Zero,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> Iterator for StepIterator<T> 
where
    T: Add<Output = T> + PartialOrd + Copy + Zero,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;

        self.current = self.current + self.step;
        
        if (self.step > T::zero() && self.current > self.end) || (self.step < T::zero() && self.current < self.end) {
            self.done = true;
        }

        Some(result)
    }
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }
