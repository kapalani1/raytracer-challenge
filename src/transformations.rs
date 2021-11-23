use crate::matrix::Matrix;

impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut translation = Matrix::identity(4);
        translation.values[0][3] = x;
        translation.values[1][3] = y;
        translation.values[2][3] = z;
        translation
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut scaling = Matrix::identity(4);
        scaling.values[0][0] = x;
        scaling.values[1][1] = y;
        scaling.values[2][2] = z;
        scaling
    }

    pub fn rotation_x(radians: f64) -> Matrix {
        let mut rotation = Matrix::identity(4);
        rotation.values[1][1] = radians.cos();
        rotation.values[1][2] = -radians.sin();
        rotation.values[2][1] = radians.sin();
        rotation.values[2][2] = radians.cos();
        rotation
    }

    pub fn rotation_y(radians: f64) -> Matrix {
        let mut rotation = Matrix::identity(4);
        rotation.values[0][0] = radians.cos();
        rotation.values[0][2] = radians.sin();
        rotation.values[2][0] = -radians.sin();
        rotation.values[2][2] = radians.cos();
        rotation
    }

    pub fn rotation_z(radians: f64) -> Matrix {
        let mut rotation = Matrix::identity(4);
        rotation.values[0][0] = radians.cos();
        rotation.values[0][1] = -radians.sin();
        rotation.values[1][0] = radians.sin();
        rotation.values[1][1] = radians.cos();
        rotation
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        let mut shearing = Matrix::identity(4);
        shearing.values[0][1] = x_y;
        shearing.values[0][2] = x_z;
        shearing.values[1][0] = y_x;
        shearing.values[1][2] = y_z;
        shearing.values[2][0] = z_x;
        shearing.values[2][1] = z_y;
        shearing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn translate() {
        let transform = Matrix::translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);
        assert_eq!(&transform * p, Tuple::point(2., 1., 7.));
        assert_eq!(&transform.inverse() * p, Tuple::point(-8., 7., 3.));
        let v = Tuple::vector(-3., 4., 5.);
        assert_eq!(&transform * v, v);
    }

    #[test]
    fn scaling() {
        let scaling = Matrix::scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);
        let v = Tuple::vector(-4., 6., 8.);
        assert_eq!(&scaling * p, Tuple::point(-8., 18., 32.));
        assert_eq!(&scaling * v, Tuple::vector(-8., 18., 32.));
        assert_eq!(&scaling.inverse() * v, Tuple::vector(-2., 2., 2.));
        assert_eq!(
            Matrix::scaling(-1., 1., 1.) * Tuple::point(2., 3., 4.),
            Tuple::point(-2., 3., 4.)
        );
    }

    #[test]
    fn rotation() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let full_quarter = Matrix::rotation_x(PI / 2.);
        assert_eq!(
            &half_quarter * p,
            Tuple::point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );
        assert_eq!(&full_quarter * p, Tuple::point(0., 0., 1.));
        assert_eq!(
            &half_quarter.inverse() * p,
            Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)
        );

        let p = Tuple::point(0., 0., 1.);
        let half_quarter = Matrix::rotation_y(PI / 4.);
        let full_quarter = Matrix::rotation_y(PI / 2.);
        assert_eq!(
            &half_quarter * p,
            Tuple::point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.)
        );
        assert_eq!(&full_quarter * p, Tuple::point(1., 0., 0.));

        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_z(PI / 4.);
        let full_quarter = Matrix::rotation_z(PI / 2.);
        assert_eq!(
            &half_quarter * p,
            Tuple::point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.)
        );
        assert_eq!(&full_quarter * p, Tuple::point(-1., 0., 0.));
    }

    #[test]
    fn shearing() {
        let shearing = Matrix::shearing(1., 0., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(&shearing * p, Tuple::point(5., 3., 4.));
        let shearing = Matrix::shearing(0., 1., 0., 0., 0., 0.);
        assert_eq!(&shearing * p, Tuple::point(6., 3., 4.));
        let shearing = Matrix::shearing(0., 0., 1., 0., 0., 0.);
        assert_eq!(&shearing * p, Tuple::point(2., 5., 4.));
        let shearing = Matrix::shearing(0., 0., 0., 1., 0., 0.);
        assert_eq!(&shearing * p, Tuple::point(2., 7., 4.));
        let shearing = Matrix::shearing(0., 0., 0., 0., 1., 0.);
        assert_eq!(&shearing * p, Tuple::point(2., 3., 6.));
        let shearing = Matrix::shearing(0., 0., 0., 0., 0., 1.);
        assert_eq!(&shearing * p, Tuple::point(2., 3., 7.));
    }

    #[test]
    fn chaining() {
        let p = Tuple::point(1., 0., 1.);
        let a = Matrix::rotation_x(PI / 2.);
        let b = Matrix::scaling(5., 5., 5.);
        let c = Matrix::translation(10., 5., 7.);

        let p2 = &a * p;
        assert_eq!(p2, Tuple::point(1., -1., 0.));
        let p3 = &b * p2;
        assert_eq!(p3, Tuple::point(5., -5., 0.));
        let p4 = &c * p3;
        assert_eq!(p4, Tuple::point(15., 0., 7.));

        assert_eq!(&c * &b * &a * p, p4);
    }
}
