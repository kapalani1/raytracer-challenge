use std::f32::NEG_INFINITY;
use std::mem::swap;

use crate::intersection::{Intersection, IntersectionList};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Object, ShapeType};
use crate::tuple::Tuple;
use crate::EPSILON;

// A unit cube
#[derive(Debug, PartialEq)]
pub struct Cube;

impl Cube {
    pub fn new(material_opt: Option<Material>) -> Object {
        let material = match material_opt {
            Some(x) => x,
            None => Material::new(),
        };

        Object {
            transform: Matrix::identity(4),
            shape: ShapeType::Cube(Cube),
            material,
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
      let tmin_numerator = -1. - origin;
      let tmax_numerator = 1. - origin;

      let mut tmin = tmin_numerator / direction;
      let mut tmax = tmax_numerator / direction;

      if tmin > tmax {
        (tmax, tmin)
      } else {
        (tmin, tmax)
      }
    }

    pub fn local_intersect<'a>(
        &self,
        ray_obj_space: &Ray,
        object: &'a Object,
    ) -> IntersectionList<'a> {
      let (xtmin, xtmax) = self.check_axis(ray_obj_space.origin.x, ray_obj_space.direction.x);
      let (ytmin, ytmax) = self.check_axis(ray_obj_space.origin.y, ray_obj_space.direction.y);
      let (ztmin, ztmax) = self.check_axis(ray_obj_space.origin.z, ray_obj_space.direction.z);
      let tmin = vec![xtmin, ytmin, ztmin].into_iter().fold(f64::INFINITY, f64::min);
      let tmax = vec![xtmax, ytmax, ztmax].into_iter().fold(f64::NEG_INFINITY, f64::max);
      IntersectionList::new(vec![Intersection::new(tmin, object), Intersection::new(tmax, object)])
    }

    pub fn local_normal_at(&self, object_space_point: Tuple) -> Tuple {
      let maxc = vec![object_space_point.x.abs(), object_space_point.y.abs(), object_space_point.z.abs()].into_iter().fold(f64::NEG_INFINITY, f64::max);

      if maxc == object_space_point.x {
        Tuple::vector(object_space_point.x, 0., 0.)
      } else if maxc == object_space_point.y {
        Tuple::vector(0., object_space_point.y, 0.)
      } else {
        Tuple::vector(0., 0., object_space_point.z)
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
    }

    #[test]
    fn normal() {
    }
}
