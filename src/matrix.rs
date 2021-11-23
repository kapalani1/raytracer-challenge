use std::ops::{Index, Mul};

use float_cmp::approx_eq;

use crate::tuple::Tuple;

#[derive(Debug, Clone)]
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

    fn rows(&self) -> usize {
        self.values.len()
    }

    fn cols(&self) -> usize {
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
        if self.rows() == 2 {
            self.values[0][0] * self.values[1][1] - self.values[1][0] * self.values[0][1]
        } else if self.rows() == 3 {
            self.values[0][0]
                * (self.values[1][1] * self.values[2][2] - self.values[2][1] * self.values[1][2])
                - self.values[0][1]
                    * (self.values[1][0] * self.values[2][2]
                        - self.values[2][0] * self.values[1][2])
                + self.values[0][2]
                    * (self.values[1][0] * self.values[2][1]
                        - self.values[2][0] * self.values[1][1])
        } else {
            self.values[0]
                .iter()
                .enumerate()
                .map(|(col, x)| x * self.cofactor(0, col))
                .collect::<Vec<f64>>()
                .iter()
                .sum()
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let row_removed: Vec<_> = self
            .values
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i != row)
            .map(|(_, row_vector)| row_vector)
            .collect();
        let values = row_removed
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .enumerate()
                    .filter(|(j, _)| *j != col)
                    .map(|(_, elem)| elem)
                    .collect()
            })
            .collect();

        Matrix { values: values }
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        match (row + col) % 2 {
            0 => self.minor(row, col),
            1 => -self.minor(row, col),
            _ => panic!("Number is neither odd nor even???"),
        }
    }

    fn is_invertible(&self) -> bool {
        !approx_eq!(f64, self.determinant(), 0.)
    }

    pub fn inverse(&self) -> Self {
        assert!(self.is_invertible());
        let mut inverse = Matrix::new(&vec![vec![0.; self.cols()]; self.rows()]);
        if self.rows() == 4 {
            // Fast path from https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
            // Appears to be significantly faster
            let a2323 =
                self.values[2][2] * self.values[3][3] - self.values[2][3] * self.values[3][2];
            let a1323 =
                self.values[2][1] * self.values[3][3] - self.values[2][3] * self.values[3][1];
            let a1223 =
                self.values[2][1] * self.values[3][2] - self.values[2][2] * self.values[3][1];
            let a0323 =
                self.values[2][0] * self.values[3][3] - self.values[2][3] * self.values[3][0];
            let a0223 =
                self.values[2][0] * self.values[3][2] - self.values[2][2] * self.values[3][0];
            let a0123 =
                self.values[2][0] * self.values[3][1] - self.values[2][1] * self.values[3][0];
            let a2313 =
                self.values[1][2] * self.values[3][3] - self.values[1][3] * self.values[3][2];
            let a1313 =
                self.values[1][1] * self.values[3][3] - self.values[1][3] * self.values[3][1];
            let a1213 =
                self.values[1][1] * self.values[3][2] - self.values[1][2] * self.values[3][1];
            let a2312 =
                self.values[1][2] * self.values[2][3] - self.values[1][3] * self.values[2][2];
            let a1312 =
                self.values[1][1] * self.values[2][3] - self.values[1][3] * self.values[2][1];
            let a1212 =
                self.values[1][1] * self.values[2][2] - self.values[1][2] * self.values[2][1];
            let a0313 =
                self.values[1][0] * self.values[3][3] - self.values[1][3] * self.values[3][0];
            let a0213 =
                self.values[1][0] * self.values[3][2] - self.values[1][2] * self.values[3][0];
            let a0312 =
                self.values[1][0] * self.values[2][3] - self.values[1][3] * self.values[2][0];
            let a0212 =
                self.values[1][0] * self.values[2][2] - self.values[1][2] * self.values[2][0];
            let a0113 =
                self.values[1][0] * self.values[3][1] - self.values[1][1] * self.values[3][0];
            let a0112 =
                self.values[1][0] * self.values[2][1] - self.values[1][1] * self.values[2][0];

            let det = self.values[0][0]
                * (self.values[1][1] * a2323 - self.values[1][2] * a1323
                    + self.values[1][3] * a1223)
                - self.values[0][1]
                    * (self.values[1][0] * a2323 - self.values[1][2] * a0323
                        + self.values[1][3] * a0223)
                + self.values[0][2]
                    * (self.values[1][0] * a1323 - self.values[1][1] * a0323
                        + self.values[1][3] * a0123)
                - self.values[0][3]
                    * (self.values[1][0] * a1223 - self.values[1][1] * a0223
                        + self.values[1][2] * a0123);
            assert!(det != 0.);
            let det = 1. / det;
            inverse.values[0][0] = det
                * (self.values[1][1] * a2323 - self.values[1][2] * a1323
                    + self.values[1][3] * a1223);
            inverse.values[0][1] = det
                * -(self.values[0][1] * a2323 - self.values[0][2] * a1323
                    + self.values[0][3] * a1223);
            inverse.values[0][2] = det
                * (self.values[0][1] * a2313 - self.values[0][2] * a1313
                    + self.values[0][3] * a1213);
            inverse.values[0][3] = det
                * -(self.values[0][1] * a2312 - self.values[0][2] * a1312
                    + self.values[0][3] * a1212);
            inverse.values[1][0] = det
                * -(self.values[1][0] * a2323 - self.values[1][2] * a0323
                    + self.values[1][3] * a0223);
            inverse.values[1][1] = det
                * (self.values[0][0] * a2323 - self.values[0][2] * a0323
                    + self.values[0][3] * a0223);
            inverse.values[1][2] = det
                * -(self.values[0][0] * a2313 - self.values[0][2] * a0313
                    + self.values[0][3] * a0213);
            inverse.values[1][3] = det
                * (self.values[0][0] * a2312 - self.values[0][2] * a0312
                    + self.values[0][3] * a0212);
            inverse.values[2][0] = det
                * (self.values[1][0] * a1323 - self.values[1][1] * a0323
                    + self.values[1][3] * a0123);
            inverse.values[2][1] = det
                * -(self.values[0][0] * a1323 - self.values[0][1] * a0323
                    + self.values[0][3] * a0123);
            inverse.values[2][2] = det
                * (self.values[0][0] * a1313 - self.values[0][1] * a0313
                    + self.values[0][3] * a0113);
            inverse.values[2][3] = det
                * -(self.values[0][0] * a1312 - self.values[0][1] * a0312
                    + self.values[0][3] * a0112);
            inverse.values[3][0] = det
                * -(self.values[1][0] * a1223 - self.values[1][1] * a0223
                    + self.values[1][2] * a0123);
            inverse.values[3][1] = det
                * (self.values[0][0] * a1223 - self.values[0][1] * a0223
                    + self.values[0][2] * a0123);
            inverse.values[3][2] = det
                * -(self.values[0][0] * a1213 - self.values[0][1] * a0213
                    + self.values[0][2] * a0113);
            inverse.values[3][3] = det
                * (self.values[0][0] * a1212 - self.values[0][1] * a0212
                    + self.values[0][2] * a0112);
        } else {
            let det = self.determinant();

            for row in 0..self.rows() {
                for col in 0..self.cols() {
                    inverse.values[col][row] = 1. / det * self.cofactor(row, col);
                }
            }
        }

        inverse
    }
}

impl<'a> Mul<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols(), rhs.rows());
        let mut values = vec![vec![0.; rhs.cols()]; self.rows()];
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

impl<'a> Mul<&'a Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'a Matrix) -> Self::Output {
        &self * rhs
    }
}

impl<'a> Mul<Tuple> for &'a Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let result = self * &Matrix::new(&vec![vec![rhs.x], vec![rhs.y], vec![rhs.z], vec![rhs.w]]);
        Tuple::from_matrix(&result)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        &self * rhs
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
                    if !approx_eq!(
                        f64,
                        self.values[row][col],
                        other.values[row][col],
                        epsilon = 0.00001
                    ) {
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
            &a * &b,
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
        assert_eq!(&a * b, Tuple::new(18., 24., 33., 1.));
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
        assert_eq!(&Matrix::identity(4) * a, a);
    }

    #[test]
    fn determinant_2x2() {
        let a = Matrix::new(&vec![vec![1., 5.], vec![-3., 2.]]);
        assert_eq!(a.determinant(), 17.);
    }

    #[test]
    fn submatrix_3x3() {
        let m = Matrix::new(&vec![
            vec![1., 5., 0.],
            vec![-3., 2., 7.],
            vec![0., 6., -3.],
        ]);
        assert_eq!(
            m.submatrix(0, 2),
            Matrix::new(&vec![vec![-3., 2.], vec![0., 6.]])
        );
    }

    #[test]
    fn submatrix_4x4() {
        let m = Matrix::new(&vec![
            vec![-6., 1., 1., 6.],
            vec![-8., 5., 8., 6.],
            vec![-1., 0., 8., 2.],
            vec![-7., 1., -1., 1.],
        ]);
        assert_eq!(
            m.submatrix(2, 1),
            Matrix::new(&vec![
                vec![-6., 1., 6.],
                vec![-8., 8., 6.],
                vec![-7., -1., 1.]
            ])
        );
    }

    #[test]
    fn minor_3x3() {
        let m = Matrix::new(&vec![
            vec![3., 5., 0.],
            vec![2., -1., -7.],
            vec![6., -1., 5.],
        ]);
        assert_eq!(m.minor(1, 0), 25.);
    }

    #[test]
    fn cofactor_3x3() {
        let m = Matrix::new(&vec![
            vec![3., 5., 0.],
            vec![2., -1., -7.],
            vec![6., -1., 5.],
        ]);
        assert_eq!(m.cofactor(1, 0), -25.);
    }

    #[test]
    fn determinant_3x3() {
        let m = Matrix::new(&vec![
            vec![1., 2., 6.],
            vec![-5., 8., -4.],
            vec![2., 6., 4.],
        ]);
        assert_eq!(m.cofactor(0, 0), 56.);
        assert_eq!(m.cofactor(0, 1), 12.);
        assert_eq!(m.cofactor(0, 2), -46.);
        assert_eq!(m.determinant(), -196.);
    }

    #[test]
    fn determinant_4x4() {
        let m = Matrix::new(&vec![
            vec![-2., -8., 3., 5.],
            vec![-3., 1., 7., 3.],
            vec![1., 2., -9., 6.],
            vec![-6., 7., 7., -9.],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.);
        assert_eq!(m.cofactor(0, 1), 447.);
        assert_eq!(m.cofactor(0, 2), 210.);
        assert_eq!(m.cofactor(0, 3), 51.);
        assert_eq!(m.determinant(), -4071.);
    }

    #[test]
    fn invertible() {
        let m = Matrix::new(&vec![
            vec![6., 4., 4., 4.],
            vec![5., 5., 7., 6.],
            vec![4., -9., 3., -7.],
            vec![9., 1., 7., -6.],
        ]);
        assert_eq!(m.determinant(), -2120.);
        assert!(m.is_invertible());
        let m = Matrix::new(&vec![
            vec![-4., 2., -2., -3.],
            vec![9., 6., 2., 6.],
            vec![0., -5., 1., -5.],
            vec![0., 0., 0., 0.],
        ]);
        assert_eq!(m.determinant(), 0.);
        assert!(!m.is_invertible());
    }

    #[test]
    fn inverse() {
        let m = Matrix::new(&vec![
            vec![-5., 2., 6., -8.],
            vec![1., -5., 1., 8.],
            vec![7., 7., -6., -7.],
            vec![1., -3., 7., 4.],
        ]);
        let b = m.inverse();
        assert_eq!(m.determinant(), 532.);
        assert_eq!(m.cofactor(2, 3), -160.);
        assert_eq!(b.values[3][2], -160. / 532.);
        assert_eq!(m.cofactor(3, 2), 105.);
        assert_eq!(b.values[2][3], 105. / 532.);
        assert_eq!(
            b,
            Matrix::new(&vec![
                vec![0.21805, 0.45113, 0.24060, -0.04511],
                vec![-0.80827, -1.45677, -0.44361, 0.52068],
                vec![-0.07895, -0.22368, -0.05263, 0.19737],
                vec![-0.52256, -0.81391, -0.30075, 0.30639]
            ])
        );

        let m = Matrix::new(&vec![
            vec![8., -5., 9., 2.],
            vec![7., 5., 6., 1.],
            vec![-6., 0., 9., 6.],
            vec![-3., 0., -9., -4.],
        ]);
        assert_eq!(
            m.inverse(),
            Matrix::new(&vec![
                vec![-0.15385, -0.15385, -0.28205, -0.53846],
                vec![-0.07692, 0.12308, 0.02564, 0.03077],
                vec![0.35897, 0.35897, 0.43590, 0.92308],
                vec![-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );

        let m = Matrix::new(&vec![
            vec![9., 3., 0., 9.],
            vec![-5., -2., -6., -3.],
            vec![-4., 9., 6., 4.],
            vec![-7., 6., 6., 2.],
        ]);
        assert_eq!(
            m.inverse(),
            Matrix::new(&vec![
                vec![-0.04074, -0.07778, 0.14444, -0.22222],
                vec![-0.07778, 0.03333, 0.36667, -0.33333],
                vec![-0.02901, -0.14630, -0.10926, 0.12963],
                vec![0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
    }

    #[test]
    fn matrix_inverse_multiply() {
        let a = Matrix::new(&vec![
            vec![3., -9., 7., 3.],
            vec![3., -8., 2., -9.],
            vec![-4., 4., 4., 1.],
            vec![-6., 5., -1., 1.],
        ]);

        let b = Matrix::new(&vec![
            vec![8., 2., 2., 2.],
            vec![3., -1., 7., 0.],
            vec![7., 0., 5., 4.],
            vec![6., -2., 0., 5.],
        ]);

        let c = &a * &b;
        assert_eq!(&c * &b.inverse(), a);
        assert_eq!(&b * &b.inverse(), Matrix::identity(4));
    }

    #[test]
    fn identity_inverse() {
        let a = Matrix::identity(4);
        assert_eq!(a.inverse(), a);
    }
}
