use std::ops::{Add, Sub};

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        // Check if matrices have the same dimensions
        if self.0.len() != rhs.0.len() {
            return None;
        }

        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
        }

        // Perform matrix addition
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            let mut row = Vec::with_capacity(self.0[i].len());
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j].clone() + rhs.0[i][j].clone());
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        // Check if matrices have the same dimensions
        if self.0.len() != rhs.0.len() {
            return None;
        }

        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
        }

        // Perform matrix subtraction
        let mut result = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            let mut row = Vec::with_capacity(self.0[i].len());
            for j in 0..self.0[i].len() {
                row.push(self.0[i][j].clone() - rhs.0[i][j].clone());
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}