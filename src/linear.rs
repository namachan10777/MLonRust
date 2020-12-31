use float_cmp::{approx_eq, F64Margin};
use std::fmt;
use std::ops;

pub struct Vector {
    inner: Vec<f64>,
}

impl Vector {
    pub fn new(inner: &[f64]) -> Self {
        Self {
            inner: inner.to_vec(),
        }
    }

    pub fn dim(&self) -> usize {
        self.inner.len()
    }

    pub fn l2norm(&self) -> f64 {
        self.inner.iter().map(|x| x * x).fold(0.0, |a, b| a + b)
    }

    pub fn norm(&self) -> f64 {
        self.l2norm().sqrt()
    }

    pub fn ap_eq(&self, rhs: &Vector) -> bool {
        assert_eq!(self.dim(), rhs.dim());
        let margin = F64Margin {
            epsilon: 0.00001,
            ulps: 5,
        };
        for i in 0..self.dim() {
            if !approx_eq!(f64, self.inner[i], rhs.inner[i], margin) {
                return false;
            }
        }
        true
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Vector")
            .field("dim", &self.dim())
            .field("in", &self.inner)
            .finish()
    }
}

impl ops::Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl ops::Mul<&Vector> for &Vector {
    type Output = f64;
    fn mul(self, rhs: &Vector) -> f64 {
        assert_eq!(self.inner.len(), rhs.inner.len());
        let mut sum = 0.0;
        for i in 0..self.inner.len() {
            sum += self.inner[i] * rhs.inner[i];
        }
        sum
    }
}

#[derive(Clone)]
pub struct Matrix {
    inner: Vec<Vec<f64>>,
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Matrix")
            .field("dim", &self.dim())
            .field("in", &self.inner)
            .finish()
    }
}

impl Matrix {
    pub fn new(inner: &[&[f64]]) -> Option<Self> {
        let mut ls = inner.iter().map(|v| v.len());
        if let Some(m) = ls.next() {
            if ls.fold(true, |acc, m2| m2 == m && acc) {
                return Some(Self {
                    inner: inner.iter().map(|v| v.to_vec()).collect::<Vec<Vec<f64>>>(),
                });
            }
        }
        None
    }

    pub fn zeros(size: (usize, usize)) -> Self {
        assert!(size.0 > 0 && size.1 > 0);
        let mut inner = Vec::new();
        let mut col = Vec::new();
        col.resize(size.1, 0.0);
        inner.resize(size.0, col);
        Self { inner }
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }

    pub fn row(&self, idx: usize) -> Vector {
        assert!(idx < self.dim().0);
        Vector {
            inner: self.inner[idx].to_owned(),
        }
    }

    pub fn col(&self, idx: usize) -> Vector {
        assert!(idx < self.dim().1);
        Vector {
            inner: self.inner.iter().map(|v| v[idx]).collect(),
        }
    }

    pub fn ap_eq(&self, rhs: &Matrix) -> bool {
        if self.dim() != rhs.dim() {
            return false;
        }
        let margin = F64Margin {
            epsilon: 0.00001,
            ulps: 5,
        };
        for i in 0..self.dim().0 {
            for j in 0..self.dim().1 {
                if !approx_eq!(f64, self.inner[i][j], rhs.inner[i][j], margin) {
                    return false;
                }
            }
        }
        true
    }
}

impl ops::Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.dim(), rhs.dim());
        let mut res = self.inner.clone();
        for (i, res_row) in res.iter_mut().enumerate() {
            for (j, res) in res_row.iter_mut().enumerate() {
                *res = rhs.inner[i][j];
            }
        }
        Matrix { inner: res }
    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.dim().0, rhs.dim().1);
        let mut res = Matrix::zeros((self.dim().0, rhs.dim().1));
        for i in 0..res.dim().0 {
            for j in 0..res.dim().1 {
                res.inner[i][j] = &self.row(i) * &rhs.col(j);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::approx_eq;
    #[test]
    fn test_norm() {
        assert!(approx_eq!(
            f64,
            Vector::new(&[1.0, 2.0, 3.0]).l2norm(),
            1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0
        ));
        assert!(approx_eq!(
            f64,
            Vector::new(&[1.0, 2.0, 3.0]).norm(),
            (1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 as f64).sqrt()
        ));
    }

    #[test]
    fn test_dot() {
        assert!(approx_eq!(
            f64,
            &Vector::new(&[1.0, 2.0, 3.0]) * &Vector::new(&[2.0, 4.0, 6.0]),
            2.0 + 8.0 + 18.0
        ));
    }

    #[test]
    fn test_ap_eq() {
        let mat1 = Matrix::new(&[&[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0]]).unwrap();
        let mat2 = Matrix::new(&[&[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0]]).unwrap();
        let v1 = Vector::new(&[1.0, 2.0, 3.0]);
        let v2 = Vector::new(&[1.0, 2.0, 3.0]);
        assert!(mat1.ap_eq(&mat2));
        assert!(v1.ap_eq(&v2));
    }

    #[test]
    fn test_mat_mul() {
        let mat1 = Matrix::new(&[&[1.0, 2.0], &[3.0, 4.0], &[5.0, 6.0]]).unwrap();
        let mat2 = Matrix::new(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]).unwrap();
        let result =
            Matrix::new(&[&[9.0, 12.0, 15.0], &[19.0, 26.0, 33.0], &[29.0, 40.0, 51.0]]).unwrap();
        assert!(result.ap_eq(&(&mat1 * &mat2)));
    }
}
