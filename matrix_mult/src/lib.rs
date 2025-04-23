use std::ops::Mul;
use std::fmt::Debug;
use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    // Returns the number of columns in the matrix
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    // Returns the number of rows in the matrix
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    // Returns the nth row in the matrix
    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.number_of_rows() {
            self.0[n].clone()
        } else {
            Vec::new()
        }
    }

    // Returns the nth column in the matrix
    pub fn col(&self, n: usize) -> Vec<T> {
        if n < self.number_of_cols() {
            self.0.iter()
                .map(|row| row[n].clone())
                .collect()
        } else {
            Vec::new()
        }
    }
}

// Implementing equality comparison for Matrix
impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// Implementing matrix multiplication
impl<T> Mul for Matrix<T> 
where 
    T: Clone + Default + Mul<Output = T> + std::iter::Sum,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let inner_dim = self.number_of_cols(); 

        let mut result = Vec::with_capacity(rows);

        for i in 0..rows {
            let mut new_row = Vec::with_capacity(cols);
            for j in 0..cols {
                // Compute the dot product of row i from self and column j from rhs
                let sum = (0..inner_dim)
                    .map(|k| {
                        let a = &self.0[i][k];
                        let b = &rhs.0[k][j];
                        a.clone() * b.clone()
                    })
                    .sum();
                new_row.push(sum);
            }
            result.push(new_row);
        }

        Some(Matrix(result))
    }
}