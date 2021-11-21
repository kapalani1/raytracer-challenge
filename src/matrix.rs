use std::ops::{Index, Mul};

use float_cmp::approx_eq;

use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Matrix {
    pub values: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: &Vec<Vec<f64>>) -> Self {
        let width = rows[0].len();
        for row in rows {
            assert_eq!(row.len(), width);
        }
        Matrix {
            values: rows.clone(),
        }
    }

    pub fn rows(&self) -> usize {
        self.values.len()
    }

    pub fn cols(&self) -> usize {
        self.values[0].len()
    }

    pub fn identity(rows: usize) -> Self {
        let mut values = vec![vec![0.; rows]; rows];
        for i in 0..rows {
            values[i][i] = 1.;
        }
        Matrix { values }
    }

    pub fn transpose(&self) -> Self {
        let mut values = vec![vec![0.; self.rows()]; self.cols()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                values[j][i] = self.values[i][j];
            }
        }

        Matrix { values }
    }

    pub fn determinant(&self) -> f64 {
        if self.rows() == 2 && self.cols() == 2 {
            self.values[0][0] * self.values[1][1] - self.values[0][1] * self.values[1][0]
        } else {
            0.
        }
    }
}

impl<'a> Mul<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols(), rhs.rows());
        let mut values = vec![vec![0.; rhs.cols()]; self.rows()];
        println!("{:?}", values);
        for row in 0..self.rows() {
            for col in 0..rhs.cols() {
                let mut val = 0.;

                for i in 0..self.cols() {
                    val += self.values[row][i] * rhs.values[i][col];
                }
                values[row][col] = val;
            }
        }

        Matrix { values }
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let result =
            &self * &Matrix::new(&vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![rhs.w]]);
        Tuple::from_matrix(&result)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0][index.1]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() || self.values[0].len() != other.values[0].len()
        {
            false
        } else {
            for row in 0..self.values.len() {
                for col in 0..self.values[0].len() {
                    if !approx_eq!(f64, self.values[row][col], other.values[row][col]) {
                        return false;
                    }
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matrix_4x4() {
        let m = Matrix::new(&vec![
            vec![1., 2., 3., 4.],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9., 10., 11., 12.],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[(0, 0)], 1.);
        assert_eq!(m[(0, 3)], 4.);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn matrix_2x2() {
        let m = Matrix::new(&vec![vec![-3., 5.], vec![1., -2.]]);
        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(0, 1)], 5.);
        assert_eq!(m[(1, 0)], 1.);
        assert_eq!(m[(1, 1)], -2.);
    }

    #[test]
    fn matrix_3x3() {
        let m = Matrix::new(&vec![
            vec![-3., 5., 0.],
            vec![1., -2., -7.],
            vec![0., 1., 1.],
        ]);
        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(1, 1)], -2.);
        assert_eq!(m[(2, 2)], 1.);
    }

    #[test]
    fn matrix_equal() {
        let a = Matrix::new(&vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ]);
        let b = Matrix::new(&vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_multiply() {
        let a = Matrix::new(&vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ]);
        let b = Matrix::new(&vec![
            vec![-2., 1., 2., 3.],
            vec![3., 2., 1., -1.],
            vec![4., 3., 6., 5.],
            vec![1., 2., 7., 8.],
        ]);
        assert_eq!(
            a * b,
            Matrix::new(&vec![
                vec![20., 22., 50., 48.],
                vec![44., 54., 114., 108.],
                vec![40., 58., 110., 102.],
                vec![16., 26., 46., 42.]
            ])
        );
    }

    #[test]
    fn matrix_tuple_multiply() {
        let a = Matrix::new(&vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ]);
        let b: Tuple = Tuple::new(1., 2., 3., 1.);
        assert_eq!(a * b, Tuple::new(18., 24., 33., 1.));
    }

    #[test]
    fn identity_matrix() {
        let a = Matrix::new(&vec![
            vec![0., 1., 2., 4.],
            vec![1., 2., 4., 8.],
            vec![2., 4., 8., 16.],
            vec![4., 8., 16., 32.],
        ]);
        assert_eq!(&a * &Matrix::identity(4), a);
    }

    #[test]
    fn identity_tuple() {
        let a = Tuple::new(1., 2., 3., 4.);
        assert_eq!(Matrix::identity(4) * a, a);
    }

    #[test]
    fn determinant_2x2() {
        let a = Matrix::new(&vec![vec![1., 5.], vec![-3., 2.]]);
        assert_eq!(a.determinant(), 17.);
    }
}
