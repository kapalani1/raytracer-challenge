use std::any::Any;

use crate::EPSILON;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Shape, ShapeIntersection, ShapeIntersectionList};
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    transform: Matrix,
    material: Material,
}

impl Plane {
    pub fn new(material: Option<Material>) -> Self {
        let material = match material {
            Some(x) => x,
            None => Material::new(),
        };

        Plane {
            transform: Matrix::identity(4),
            material,
        }
    }
}

impl Shape for Plane {
    fn normal_at(&self, point: Tuple) -> Tuple {
        assert!(point.is_point());
        let object_normal = Tuple::vector(0., 1., 0.);

        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn intersect(&self, ray: &Ray) -> ShapeIntersectionList {
      let ray_obj_space = ray.transform(&self.transform.inverse());
      if ray_obj_space.direction.y.abs() < EPSILON {
        ShapeIntersectionList::new(vec![])
      } else {
        let t = -ray_obj_space.origin.y / ray_obj_space.direction.y;
        ShapeIntersectionList::new(vec![ShapeIntersection::new(t, self, Some(ray))])
      }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix {
      &self.transform
    }

    fn set_transform(&mut self, m: &Matrix) {
      self.transform = m.clone();
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersect() {
    let p = Plane::new(None);
    let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0. ,1.));
    let i = r.intersect(&p);
    assert_eq!(i.intersections.len(), 0);

    let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
    let i = r.intersect(&p);
    assert_eq!(i.intersections.len(), 0);

    let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));
    let i = r.intersect(&p);
    assert_eq!(i.intersections.len(), 1);
    assert_eq!(i.intersections[0].t, 1.);
    assert!(std::ptr::eq(i.intersections[0].shape.as_any(), p.as_any()));

    let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));
    let i = r.intersect(&p);
    assert_eq!(i.intersections.len(), 1);
    assert_eq!(i.intersections[0].t, 1.);
    assert!(std::ptr::eq(i.intersections[0].shape.as_any(), p.as_any()));
  }
}
