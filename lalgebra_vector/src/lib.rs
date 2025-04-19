use std::fmt::Debug;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + Clone + PartialEq + Eq {}
impl<T> Scalar for T where T: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + Clone + PartialEq + Eq {}

impl<T: Scalar> Add for Vector<T> {
	type Output = Option<Self>;

	fn add(self, rhs: Self) -> Self::Output {
		if self.0.len() != rhs.0.len() {
			return None;
		}
		let result = self.0.iter().zip(rhs.0.iter())
			.map(|(a, b)| *a + *b)
			.collect();
		Some(Vector(result))
	}
}

impl<T: Scalar> Vector<T> {
	pub fn new() -> Self {
		Vector(Vec::new())
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
		if self.0.len() != other.0.len() {
			return None;
		}
		let mut result = self.0[0] * other.0[0];
		for i in 1..self.0.len() {
			result = result + self.0[i] * other.0[i];
		}
		Some(result)
	}
}