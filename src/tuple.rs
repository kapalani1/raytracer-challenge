use std::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
    vec,
};

use float_cmp::approx_eq;

use crate::matrix::Matrix;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1. }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0. }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        self.clone() / self.magnitude()
    }

    pub fn dot(&self, rhs: &Tuple) -> f64 {
        assert!(rhs.is_vector());
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Tuple) -> Self {
        assert!(rhs.is_vector());
        Tuple::vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn from_matrix(m: &Matrix) -> Self {
        assert_eq!(m.values.len(), 4);
        assert_eq!(m.values[0].len(), 1);
        Tuple {
            x: m.values[0][0],
            y: m.values[1][0],
            z: m.values[2][0],
            w: m.values[3][0],
        }
    }

    pub fn to_vector(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z, self.w]
    }

    pub fn reflect(&self, normal: &Tuple) -> Self {
        assert!(self.is_vector());
        *self - *normal * 2. * self.dot(normal)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x, epsilon = 0.00001)
            && approx_eq!(f64, self.y, other.y, epsilon = 0.00001)
            && approx_eq!(f64, self.z, other.z, epsilon = 0.00001)
            && approx_eq!(f64, self.w, other.w, epsilon = 0.00001)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Tuple> for Tuple {
    fn add_assign(&mut self, rhs: Tuple) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_tuple() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn vector_tuple() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn add_tuples() {
        let a1 = Tuple::new(3., -2., 5., 1.);
        let a2 = Tuple::new(-2., 3., 1., 0.);
        assert_eq!(a1 + a2, Tuple::new(1., 1., 6., 1.));
    }

    #[test]
    fn negate_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(-a, Tuple::new(-1., 2., -3., 4.));
    }

    #[test]
    fn multiply_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn multiply_fraction() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn divide_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert_eq!(a / 2., Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn point() {
        let p = Tuple::point(4., -4., 3.);
        assert_eq!(p, Tuple::new(4., -4., 3., 1.));
    }

    #[test]
    fn vector() {
        let v = Tuple::vector(4., -4., 3.);
        assert_eq!(v, Tuple::new(4., -4., 3., 0.));
    }

    #[test]
    fn subtract_points() {
        let p1 = Tuple::point(3., 2., 1.);
        let p2 = Tuple::point(5., 6., 7.);
        assert_eq!(p1 - p2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);
        assert_eq!(p - v, Tuple::point(-2., -4., -6.));
    }

    #[test]
    fn subtract_vectors() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);
        assert_eq!(v1 - v2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn subtract_vector_from_zero() {
        let zero = Tuple::vector(0., 0., 0.);
        let v = Tuple::vector(1., -2., 3.);
        assert_eq!(zero - v, Tuple::vector(-1., 2., -3.));
    }

    #[test]
    fn magnitude() {
        let v = Tuple::vector(1., 0., 0.);
        assert_eq!(v.magnitude(), 1.);
        let v = Tuple::vector(0., 1., 0.);
        assert_eq!(v.magnitude(), 1.);
        let v = Tuple::vector(0., 0., 1.);
        assert_eq!(v.magnitude(), 1.);
        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.magnitude(), 14_f64.sqrt());
        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.normalize().magnitude(), 1.);
    }

    #[test]
    fn dot() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.dot(&b), 20.);
    }

    #[test]
    fn cross() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.cross(&b), Tuple::vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), Tuple::vector(1., -2., 1.));
    }

    #[test]
    fn reflect() {
        let v = Tuple::vector(1., -1., 0.);
        let n = Tuple::vector(0., 1., 0.);
        assert_eq!(v.reflect(&n), Tuple::vector(1., 1., 0.));

        let v = Tuple::vector(0., -1., 0.);
        let n = Tuple::vector(2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.);
        assert_eq!(v.reflect(&n), Tuple::vector(1., 0., 0.));
    }
}
