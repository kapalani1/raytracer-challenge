use crate::intersection::{Intersect, Intersection, IntersectionList};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f64,
    pub transform: Matrix
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
            transform: Matrix::identity(4)
        }
    }

    pub fn set_transform(&mut self, m: &Matrix) {
      self.transform = m.clone();
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> IntersectionList {
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            IntersectionList::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            IntersectionList::new(vec![
                Intersection::new(t1, self),
                Intersection::new(t2, self),
            ])
        }
    }

    fn as_ref(&self) -> &dyn Intersect {
        self
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sphere() {
    let mut s = Sphere::new();
    assert_eq!(s.transform, Matrix::identity(4));
    let m = Matrix::translation(2., 3., 4.);
    s.set_transform(&m);
    assert_eq!(s.transform, m);
  }

  #[test]
  fn ray_sphere_intersection() {
    let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
    let mut s = Sphere::new();
    s.set_transform(&Matrix::scaling(2., 2., 2.));
    let i = r.intersect(&s);
    assert_eq!(i.intersections.len(), 2);
    assert_eq!(i.intersections[0].t, 3.);
    assert_eq!(i.intersections[1].t, 7.);

    s.set_transform(&Matrix::translation(5., 0., 0.));
    let i = r.intersect(&s);
    assert_eq!(i.intersections.len(), 0);
  }
}
