use crate::intersection::{Intersection, IntersectionList};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Object, ShapeType};
use crate::tuple::Tuple;
use crate::EPSILON;

// An XZ plane
#[derive(Debug, PartialEq)]
pub struct Plane;

impl Plane {
    pub fn new(material_opt: Option<Material>) -> Object {
        let material = match material_opt {
            Some(x) => x,
            None => Material::new(),
        };

        Object {
            transform: Matrix::identity(4),
            shape: ShapeType::Plane(Plane),
            material,
        }
    }

    pub fn local_intersect<'a>(
        &self,
        ray_obj_space: &Ray,
        object: &'a Object,
    ) -> IntersectionList<'a> {
        if ray_obj_space.direction.y.abs() < EPSILON {
            IntersectionList::new(vec![])
        } else {
            let t = -ray_obj_space.origin.y / ray_obj_space.direction.y;
            IntersectionList::new(vec![Intersection::new(t, object)])
        }
    }

    pub fn local_normal_at(&self, _object_space_point: Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let p = Plane::new(None);
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));
        let i = r.intersect_object(&p);
        assert_eq!(i.intersections.len(), 0);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let i = r.intersect_object(&p);
        assert_eq!(i.intersections.len(), 0);

        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));
        let i = r.intersect_object(&p);
        assert_eq!(i.intersections.len(), 1);
        assert_eq!(i.intersections[0].t, 1.);
        assert!(std::ptr::eq(i.intersections[0].object, &p));

        let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));
        let i = r.intersect_object(&p);
        assert_eq!(i.intersections.len(), 1);
        assert_eq!(i.intersections[0].t, 1.);
        assert!(std::ptr::eq(i.intersections[0].object, &p));
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
