#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar<Item = Self> {
    type Item;
    
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for u32 {
    type Item = u32;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl<T: Scalar<Item = T>> Matrix<T> {
    // Creates a new 1x1 matrix
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    // Creates a matrix of specified dimensions filled with zeros
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut result = Vec::with_capacity(row);
        for _ in 0..row {
            let mut row_vec = Vec::with_capacity(col);
            for _ in 0..col {
                row_vec.push(T::zero());
            }
            result.push(row_vec);
        }
        Matrix(result)
    }

    // Creates an identity matrix of size n×n
    pub fn identity(n: usize) -> Matrix<T> {
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let mut row_vec = Vec::with_capacity(n);
            for j in 0..n {
                if i == j {
                    row_vec.push(T::one());
                } else {
                    row_vec.push(T::zero());
                }
            }
            result.push(row_vec);
        }
        Matrix(result)
    }
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}