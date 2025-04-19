use std::ops::Add;

// Define the Scalar trait for types that can be used with Vector
pub trait Scalar: Add<Output = Self> + std::fmt::Debug + Clone + PartialEq + Eq + std::ops::Mul<Output = Self> + std::marker::Sized {}

// Implement Scalar for common numeric types
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for f32 {}
impl Scalar for f64 {}

// Vector struct that contains a Vec of elements of type T (which must implement Scalar)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

// Implementation of Add for Vector
impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        // Check if vectors have the same length
        if self.0.len() != other.0.len() {
            return None;
        }

        // Add corresponding elements
        let result = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        // Create a new empty vector
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        // Check if vectors have the same length
        if self.0.len() != other.0.len() {
            return None;
        }
        
        // Calculate dot product: sum of element-wise products
        let result = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| a.clone() * b.clone())
            .fold(None, |acc, item| {
                match acc {
                    None => Some(item),
                    Some(val) => Some(val + item),
                }
            });
            
        result
    }
}