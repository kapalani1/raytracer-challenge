use crate::intersection::{Intersect, IntersectionList};
use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    pub fn intersect<'a>(&self, object: &'a impl Intersect) -> IntersectionList<'a> {
        object.intersect(&self)
    }

    pub fn transform(&self, transformation: &Matrix) -> Self {
        let origin = transformation * self.origin;
        let direction = transformation * self.direction;
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);

        let r = Ray::new(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));
        assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));
        assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));
        assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn ray_sphere_intersect() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 4.);
        assert_eq!(i.intersections[1].t, 6.);
        assert!(std::ptr::eq(
            *i.intersections[0].object.as_ref(),
            s.as_ref()
        ));
        assert!(std::ptr::eq(
            *i.intersections[1].object.as_ref(),
            s.as_ref()
        ));

        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 5.);
        assert_eq!(i.intersections[1].t, 5.);
        assert!(std::ptr::eq(
            *i.intersections[0].object.as_ref(),
            s.as_ref()
        ));
        assert!(std::ptr::eq(
            *i.intersections[1].object.as_ref(),
            s.as_ref()
        ));

        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 0);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, -1.);
        assert_eq!(i.intersections[1].t, 1.);
        assert!(std::ptr::eq(
            *i.intersections[0].object.as_ref(),
            s.as_ref()
        ));
        assert!(std::ptr::eq(
            *i.intersections[1].object.as_ref(),
            s.as_ref()
        ));

        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, -6.);
        assert_eq!(i.intersections[1].t, -4.);
        assert!(std::ptr::eq(
            *i.intersections[0].object.as_ref(),
            s.as_ref()
        ));
        assert!(std::ptr::eq(
            *i.intersections[1].object.as_ref(),
            s.as_ref()
        ));
    }

    #[test]
    fn transform() {
        let r = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let m = Matrix::translation(3., 4., 5.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(4., 6., 8.));
        assert_eq!(r2.direction, Tuple::vector(0., 1., 0.));

        let m = Matrix::scaling(2., 3., 4.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(2., 6., 12.));
        assert_eq!(r2.direction, Tuple::vector(0., 3., 0.));
    }
}
