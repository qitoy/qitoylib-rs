use qitoy_ring::Ring;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug)]
pub struct Matrix<T: Ring> {
    row: usize,
    column: usize,
    mat: Vec<Vec<T::S>>,
}

impl<T: Ring> Matrix<T> {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            mat: vec![vec![T::zero(); column]; row],
        }
    }
    pub fn identity(n: usize) -> Self {
        let mut mat = Self::new(n, n);
        for i in 0..n {
            mat[(i, i)] = T::one();
        }
        mat
    }
    pub fn row(&self) -> usize {
        self.row
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn pow(mut self, mut rhs: u64) -> Self {
        assert_eq!(self.row, self.column);
        let mut ret = Self::identity(self.row);
        while rhs > 0 {
            if rhs & 1 == 1 {
                ret *= self.clone();
            }
            self *= self.clone();
            rhs >>= 1;
        }
        ret
    }
}

impl<T: Ring> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            column: self.column,
            mat: self.mat.clone(),
        }
    }
}

impl<T: Ring> From<Vec<Vec<T::S>>> for Matrix<T> {
    fn from(value: Vec<Vec<T::S>>) -> Self {
        let row = value.len();
        if row == 0 {
            return Self {
                row,
                column: 0,
                mat: vec![],
            };
        }
        let column = value[0].len();
        assert!(value.iter().all(|v| v.len() == column));
        Self {
            row,
            column,
            mat: value,
        }
    }
}

impl<T: Ring> Index<(usize, usize)> for Matrix<T> {
    type Output = T::S;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.mat[index.0][index.1]
    }
}

impl<T: Ring> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.mat[index.0][index.1]
    }
}

impl<T: Ring> Add for Matrix<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.column, rhs.column);
        for i in 0..self.row {
            for j in 0..self.column {
                self[(i, j)] = T::add(&self[(i, j)], &rhs[(i, j)]);
            }
        }
        self
    }
}

impl<T: Ring> Neg for Matrix<T> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for i in 0..self.row {
            for j in 0..self.column {
                self[(i, j)] = T::neg(&self[(i, j)]);
            }
        }
        self
    }
}

impl<T: Ring> Sub for Matrix<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: Ring> Mul for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.column, rhs.row);
        let mut ret = Matrix {
            row: self.row,
            column: rhs.column,
            mat: vec![vec![T::zero(); rhs.column]; self.row],
        };
        for i in 0..self.row {
            for k in 0..self.column {
                for j in 0..rhs.column {
                    ret[(i, j)] = T::add(&ret[(i, j)], &T::mul(&self[(i, k)], &rhs[(k, j)]));
                }
            }
        }
        ret
    }
}

impl<T: Ring> AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<T: Ring> SubAssign for Matrix<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<T: Ring> MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}
