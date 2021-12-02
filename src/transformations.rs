use crate::{matrix::Matrix, tuple::Tuple};

impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::new(&vec![
            vec![1., 0., 0., x],
            vec![0., 1., 0., y],
            vec![0., 0., 1., z],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::new(&vec![
            vec![x, 0., 0., 0.],
            vec![0., y, 0., 0.],
            vec![0., 0., z, 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn rotation_x(radians: f64) -> Matrix {
        Matrix::new(&vec![
            vec![1., 0., 0., 0.],
            vec![0., radians.cos(), -radians.sin(), 0.],
            vec![0., radians.sin(), radians.cos(), 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn rotation_y(radians: f64) -> Matrix {
        Matrix::new(&vec![
            vec![radians.cos(), 0., radians.sin(), 0.],
            vec![0., 1., 0., 0.],
            vec![-radians.sin(), 0., radians.cos(), 0.],
            vec![0., 0., 0., 1.],
        ])
    }

    pub fn rotation_z(radians: f64) -> Matrix {
      Matrix::new(&vec![
        vec![radians.cos(), -radians.sin(), 0., 0.],
        vec![radians.sin(), radians.cos(), 0., 0.],
        vec![0., 0., 1., 0.],
        vec![0., 0., 0., 1.],
      ])
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        Matrix::new(&vec![
          vec![1., x_y, x_z, 0.],
          vec![y_x, 1., y_z, 0.],
          vec![z_x, z_y, 1., 0.],
          vec![0., 0., 0., 1.]
        ])
    }

    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
        assert!(from.is_point());
        assert!(to.is_point());
        assert!(up.is_vector());
        let forward = (to - from).normalize();
        let left = forward.cross(&up.normalize());
        let true_up = left.cross(&forward);
        Matrix::new(&vec![
            vec![left.x, left.y, left.z, 0.],
            vec![true_up.x, true_up.y, true_up.z, 0.],
            vec![-forward.x, -forward.y, -forward.z, 0.],
            vec![0., 0., 0., 1.],
        ]) * &Matrix::translation(-from.x, -from.y, -from.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tuple::Tuple, PI};

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

    #[test]
    fn view_transform() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::vector(0., 1., 0.);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::identity(4));

        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., 1.);
        let up = Tuple::vector(0., 1., 0.);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::scaling(-1., 1., -1.));

        let from = Tuple::point(0., 0., 8.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::translation(0., 0., -8.));

        let from = Tuple::point(1., 3., 2.);
        let to = Tuple::point(4., -2., 8.);
        let up = Tuple::vector(1., 1., 0.);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(
            t,
            Matrix::new(&vec![
                vec![-0.50709, 0.50709, 0.67612, -2.36643],
                vec![0.76772, 0.60609, 0.12122, -2.82843],
                vec![-0.35857, 0.59761, -0.71714, 0.00000],
                vec![0.00000, 0.00000, 0.00000, 1.00000],
            ])
        );
    }
}
