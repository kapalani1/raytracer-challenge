use std::any::Any;

use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Shape, Object, ShapeType, ObjectIntersectionList, ObjectIntersection};
use crate::tuple::Tuple;
use crate::EPSILON;

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

    pub fn object_new(material_opt: Option<Material>) -> Object {
      let material = match material_opt {
          Some(x) => x,
          None => Material::new(),
      };

      Object {
          transform: Matrix::identity(4),
          shape: ShapeType::Plane(Plane::new(Some(material.clone()))),
          material,
      }
  }

  pub fn local_object_intersect<'a>(
      &self,
      ray_obj_space: &Ray,
      object: &'a Object,
  ) -> ObjectIntersectionList<'a> {
    if ray_obj_space.direction.y.abs() < EPSILON {
      ObjectIntersectionList::new(vec![])
  } else {
      let t = -ray_obj_space.origin.y / ray_obj_space.direction.y;
      ObjectIntersectionList::new(vec![ObjectIntersection::new(t, object)])
  }
}

  pub fn local_object_normal(&self, object_space_point: Tuple) -> Tuple {
    Tuple::vector(0., 1., 0.)
  }
}

impl Shape for Plane {
    fn local_normal(&self, _point: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_intersect(&self, ray_obj_space: &Ray) -> Vec<(f64, &dyn Shape)> {
        if ray_obj_space.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray_obj_space.origin.y / ray_obj_space.direction.y;
            vec![(t, self)]
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
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));
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

    #[test]
    fn normal() {
        let p = Plane::new(None);
        let n1 = p.normal_at(Tuple::point(0., 0., 0.));
        let n2 = p.normal_at(Tuple::point(10., 0., -10.));
        let n3 = p.normal_at(Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));
    }
}
