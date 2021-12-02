use float_cmp::approx_eq;

use crate::EPSILON;
use crate::intersection::{Intersection, IntersectionList};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Object, ShapeType};
use crate::tuple::Tuple;

// A unit cube
#[derive(Debug, PartialEq)]
pub struct Cylinder {
  minimum: f64,
  maximum: f64,
}

impl Cylinder {
    pub fn new(material_opt: Option<Material>) -> Object {
        let material = match material_opt {
            Some(x) => x,
            None => Material::new(),
        };

        Object {
            transform: Matrix::identity(4),
            shape: ShapeType::Cylinder(Cylinder {
              minimum: -f64::NEG_INFINITY,
              maximum: f64::INFINITY
            }),
            material,
        }
    }

    pub fn local_intersect<'a>(
        &self,
        ray_obj_space: &Ray,
        object: &'a Object,
    ) -> IntersectionList<'a> {
      let a = ray_obj_space.direction.x * ray_obj_space.direction.x + ray_obj_space.direction.z * ray_obj_space.direction.z;

      if approx_eq!(f64, a, 0., epsilon = EPSILON) {
        return IntersectionList::new(vec![]);
      }

      let b = 2. * ray_obj_space.origin.x * ray_obj_space.direction.x + 2. * ray_obj_space.origin.z * ray_obj_space.direction.z;
      let c = ray_obj_space.origin.x * ray_obj_space.origin.x + ray_obj_space.origin.z * ray_obj_space.origin.z - 1.;
      let discriminant = b*b - 4. * a * c;

      if discriminant < 0. {
        return IntersectionList::new(vec![])
      }

      let t0 = -b - discriminant.sqrt() / (2. * a);
      let t1 = -b + discriminant.sqrt() / (2. * a);

      return IntersectionList::new(vec![Intersection::new(t0, object), Intersection::new(t1, object)]);
    }

    pub fn local_normal_at(&self, object_space_point: Tuple) -> Tuple {
        Tuple::vector(object_space_point.x, 0., object_space_point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {}

    #[test]
    fn misses() {}

    #[test]
    fn normal() {}
}
